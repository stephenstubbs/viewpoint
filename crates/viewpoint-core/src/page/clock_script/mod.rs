//! Clock mocking JavaScript library.
//!
//! This module contains the JavaScript code that gets injected into pages
//! to enable clock mocking functionality.

/// JavaScript code for clock mocking library.
/// This gets injected into the page and provides clock control functions.
pub const CLOCK_MOCK_SCRIPT: &str = r"
(function() {
    // Only install once
    if (window.__viewpointClock) return;
    
    // Store original functions
    const originalDate = Date;
    const originalSetTimeout = window.setTimeout;
    const originalSetInterval = window.setInterval;
    const originalClearTimeout = window.clearTimeout;
    const originalClearInterval = window.clearInterval;
    const originalRequestAnimationFrame = window.requestAnimationFrame;
    const originalCancelAnimationFrame = window.cancelAnimationFrame;
    const originalPerformanceNow = performance.now.bind(performance);
    
    // Clock state
    let installed = false;
    let currentTime = Date.now();
    let fixedTime = null; // null means time flows, number means frozen
    let timerId = 0;
    let timers = new Map(); // id -> { callback, time, interval, type }
    let rafId = 0;
    let rafCallbacks = new Map(); // id -> callback
    let paused = false;
    let startRealTime = null;
    let startMockedTime = null;
    
    // Mock Date
    class MockDate extends originalDate {
        constructor(...args) {
            if (args.length === 0) {
                super(window.__viewpointClock.now());
            } else {
                super(...args);
            }
        }
        
        static now() {
            return window.__viewpointClock.now();
        }
        
        static parse(str) {
            return originalDate.parse(str);
        }
        
        static UTC(...args) {
            return originalDate.UTC(...args);
        }
    }
    
    // Mock setTimeout
    function mockSetTimeout(callback, delay = 0, ...args) {
        const id = ++timerId;
        const executeTime = window.__viewpointClock.now() + delay;
        timers.set(id, {
            callback: () => callback(...args),
            time: executeTime,
            interval: null,
            type: 'timeout'
        });
        return id;
    }
    
    // Mock setInterval
    function mockSetInterval(callback, delay = 0, ...args) {
        const id = ++timerId;
        const executeTime = window.__viewpointClock.now() + delay;
        timers.set(id, {
            callback: () => callback(...args),
            time: executeTime,
            interval: delay,
            type: 'interval'
        });
        return id;
    }
    
    // Mock clearTimeout
    function mockClearTimeout(id) {
        timers.delete(id);
    }
    
    // Mock clearInterval
    function mockClearInterval(id) {
        timers.delete(id);
    }
    
    // Mock requestAnimationFrame
    function mockRequestAnimationFrame(callback) {
        const id = ++rafId;
        rafCallbacks.set(id, callback);
        return id;
    }
    
    // Mock cancelAnimationFrame
    function mockCancelAnimationFrame(id) {
        rafCallbacks.delete(id);
    }
    
    // Mock performance.now
    function mockPerformanceNow() {
        if (!installed) return originalPerformanceNow();
        // performance.now() is relative to page load, so we offset from start
        return window.__viewpointClock.now() - startMockedTime;
    }
    
    // Process due timers
    function processTimers(upToTime) {
        let processed = 0;
        const maxIterations = 10000; // Safety limit
        
        for (let i = 0; i < maxIterations; i++) {
            // Find the next timer that should fire
            let nextTimer = null;
            let nextId = null;
            
            for (const [id, timer] of timers) {
                if (timer.time <= upToTime) {
                    if (!nextTimer || timer.time < nextTimer.time) {
                        nextTimer = timer;
                        nextId = id;
                    }
                }
            }
            
            if (!nextTimer) break;
            
            // Update current time to timer's time
            currentTime = nextTimer.time;
            
            // Execute the timer
            try {
                nextTimer.callback();
            } catch (e) {
                console.error('Timer callback error:', e);
            }
            
            processed++;
            
            // Handle interval vs timeout
            if (nextTimer.type === 'interval' && nextTimer.interval > 0) {
                // Reschedule interval
                nextTimer.time = nextTimer.time + nextTimer.interval;
            } else {
                // Remove timeout
                timers.delete(nextId);
            }
        }
        
        // Update to final time
        currentTime = upToTime;
        return processed;
    }
    
    // Process animation frames
    function processAnimationFrames() {
        const callbacks = Array.from(rafCallbacks.entries());
        rafCallbacks.clear();
        
        for (const [id, callback] of callbacks) {
            try {
                callback(window.__viewpointClock.now());
            } catch (e) {
                console.error('RAF callback error:', e);
            }
        }
        
        return callbacks.length;
    }
    
    // Clock API exposed to window
    window.__viewpointClock = {
        // Get current mocked time
        now() {
            if (!installed) return originalDate.now();
            if (fixedTime !== null) return fixedTime;
            if (paused) return currentTime;
            
            // Time flows from the point we set it
            if (startRealTime !== null) {
                const elapsed = originalDate.now() - startRealTime;
                return startMockedTime + elapsed;
            }
            
            return currentTime;
        },
        
        // Install clock mocking
        install() {
            if (installed) return;
            installed = true;
            currentTime = originalDate.now();
            startMockedTime = currentTime;
            startRealTime = originalDate.now();
            
            window.Date = MockDate;
            window.setTimeout = mockSetTimeout;
            window.setInterval = mockSetInterval;
            window.clearTimeout = mockClearTimeout;
            window.clearInterval = mockClearInterval;
            window.requestAnimationFrame = mockRequestAnimationFrame;
            window.cancelAnimationFrame = mockCancelAnimationFrame;
            performance.now = mockPerformanceNow;
        },
        
        // Uninstall clock mocking
        uninstall() {
            if (!installed) return;
            installed = false;
            fixedTime = null;
            paused = false;
            timers.clear();
            rafCallbacks.clear();
            
            window.Date = originalDate;
            window.setTimeout = originalSetTimeout;
            window.setInterval = originalSetInterval;
            window.clearTimeout = originalClearTimeout;
            window.clearInterval = originalClearInterval;
            window.requestAnimationFrame = originalRequestAnimationFrame;
            window.cancelAnimationFrame = originalCancelAnimationFrame;
            performance.now = originalPerformanceNow;
        },
        
        // Set fixed time (frozen)
        setFixedTime(timestamp) {
            fixedTime = typeof timestamp === 'string' 
                ? new originalDate(timestamp).getTime() 
                : timestamp;
            currentTime = fixedTime;
            paused = true;
        },
        
        // Set system time (flows normally from this point)
        setSystemTime(timestamp) {
            const time = typeof timestamp === 'string' 
                ? new originalDate(timestamp).getTime() 
                : timestamp;
            fixedTime = null;
            currentTime = time;
            startMockedTime = time;
            startRealTime = originalDate.now();
            paused = false;
        },
        
        // Run for a duration (advancing time and firing timers)
        runFor(ms) {
            if (!installed) return 0;
            const targetTime = this.now() + ms;
            const processed = processTimers(targetTime);
            processAnimationFrames();
            return processed;
        },
        
        // Fast forward (just advance time without processing timers)
        fastForward(ms) {
            if (!installed) return;
            currentTime = this.now() + ms;
            if (fixedTime !== null) {
                fixedTime = currentTime;
            } else {
                startMockedTime = currentTime;
                startRealTime = originalDate.now();
            }
        },
        
        // Pause at a specific time
        pauseAt(timestamp) {
            const time = typeof timestamp === 'string' 
                ? new originalDate(timestamp).getTime() 
                : timestamp;
            currentTime = time;
            fixedTime = time;
            paused = true;
        },
        
        // Resume normal time flow
        resume() {
            if (!paused) return;
            paused = false;
            fixedTime = null;
            startMockedTime = currentTime;
            startRealTime = originalDate.now();
        },
        
        // Run all pending timers
        runAllTimers() {
            if (!installed) return 0;
            const maxTime = Math.max(...Array.from(timers.values()).map(t => t.time), this.now());
            const processed = processTimers(maxTime);
            processAnimationFrames();
            return processed;
        },
        
        // Run to the last scheduled timer
        runToLast() {
            if (!installed || timers.size === 0) return 0;
            const lastTime = Math.max(...Array.from(timers.values()).map(t => t.time));
            const processed = processTimers(lastTime);
            processAnimationFrames();
            return processed;
        },
        
        // Get pending timer count
        pendingTimerCount() {
            return timers.size + rafCallbacks.size;
        },
        
        // Check if installed
        isInstalled() {
            return installed;
        }
    };
})();
";
