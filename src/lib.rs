#[derive(Eq,PartialEq,Debug)]
enum Command {
    Publish(String),
    Retrieve,
}

#[derive(Eq,PartialEq,Debug)]
enum Error {
    UnknownVerb,
    UnexpectedPayload,
    MissingPayload,
    EmptyMessage,
    IncompleteMessage,
}

fn parse(input: &str) -> Result<Command, Error> {
    let mut input = input;
    if let Some(pos) = input.find("\n") {
        input = &input[0..pos];
    } else {
        return Err(Error::IncompleteMessage);
    }
    let mut split = input.splitn(2, " ");

    if let Some(verb) = split.next() {
        match verb.trim() {
            "RETRIEVE" => {
                if split.next() == None {
                    Ok(Command::Retrieve)
                } else {
                    Err(Error::UnexpectedPayload)
                }
            }
            "PUBLISH" => {
                if let Some(payload) = split.next() {
                    Ok(Command::Publish(payload.trim().to_string()))
                } else {
                    Err(Error::MissingPayload)
                }
            }
            _ => Err(Error::UnknownVerb),
        }
    } else {
        Err(Error::EmptyMessage)
    }
}

#[test]
fn test_publish() {
    let line = "PUBLISH TestMessage\n";
    let result = parse(line);
    assert_eq!(result, Ok(Command::Publish("TestMessage".into())));
}

#[test]
fn test_empty_string() {
    let line = "";
    let result = parse(line);
    assert_eq!(result, Err(Error::IncompleteMessage));
}

#[test]
fn test_missing_newline() {
    let line = "FooBar";
    let result = parse(line);
    assert_eq!(result, Err(Error::IncompleteMessage));
}

#[test]
fn test_retrieve_with_payload() {
    let line = "RETRIEVE payload\n";
    let result = parse(line);
    assert_eq!(result, Err(Error::UnexpectedPayload));
}

#[test]
fn test_publish_without_payload() {
    let line = "PUBLISH\n";
    let result = parse(line);
    assert_eq!(result, Err(Error::MissingPayload));
}
