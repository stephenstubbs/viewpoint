## MODIFIED Requirements

### Requirement: Set Input Files

The system SHALL allow setting files on file input elements.

#### Scenario: Set single file

- **GIVEN** a file input element
- **WHEN** `locator.set_input_files("file.txt").await` is called
- **THEN** the file is set on the input

#### Scenario: Set multiple files

- **GIVEN** a file input with multiple attribute
- **WHEN** `locator.set_input_files(vec!["a.txt", "b.txt"]).await` is called
- **THEN** both files are set on the input

#### Scenario: Clear files

- **GIVEN** a file input with files set
- **WHEN** `locator.set_input_files(vec![]).await` is called
- **THEN** the files are cleared

#### Scenario: Set file buffer

- **GIVEN** a file input element
- **WHEN** `locator.set_input_files(FilePayload::new("test.txt", "text/plain", bytes)).await` is called
- **THEN** the file is uploaded from memory with the specified name and content

#### Scenario: Set multiple file buffers

- **GIVEN** a file input with multiple attribute
- **WHEN** `locator.set_input_files(vec![file1, file2]).await` is called with FilePayload objects
- **THEN** both files are uploaded from memory

#### Scenario: Set file via ref from aria snapshot

- **GIVEN** an aria snapshot containing a file input element with ref `c0p0e5`
- **WHEN** `page.locator_from_ref("c0p0e5").set_input_files(&["file.txt"]).await` is called
- **THEN** the file is set on the input

#### Scenario: Set file buffer via ref from aria snapshot

- **GIVEN** an aria snapshot containing a file input element with ref `c0p0e5`
- **WHEN** `page.locator_from_ref("c0p0e5").set_input_files_from_buffer(&[payload]).await` is called
- **THEN** the file is uploaded from memory
