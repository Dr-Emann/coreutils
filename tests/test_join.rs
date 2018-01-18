use common::util::*;


#[test]
fn empty_files() {
    new_ucmd!()
        .arg("empty.txt")
        .arg("empty.txt")
        .succeeds().stdout_only("");

    new_ucmd!()
        .arg("empty.txt")
        .arg("fields_1.txt")
        .succeeds().stdout_only("");

    new_ucmd!()
        .arg("fields_1.txt")
        .arg("empty.txt")
        .succeeds().stdout_only("");
}

#[test]
fn empty_intersection() {
    new_ucmd!()
        .arg("fields_1.txt")
        .arg("fields_2.txt")
        .arg("-2")
        .arg("2")
        .succeeds().stdout_only("");
}

#[test]
fn default_arguments() {
    new_ucmd!()
        .arg("fields_1.txt")
        .arg("fields_2.txt")
        .succeeds().stdout_only_fixture("default.expected");
}

#[test]
fn different_fields() {
    new_ucmd!()
        .arg("fields_2.txt")
        .arg("fields_4.txt")
        .arg("-j")
        .arg("2")
        .succeeds().stdout_only_fixture("different_fields.expected");

    new_ucmd!()
        .arg("fields_2.txt")
        .arg("fields_4.txt")
        .arg("-1")
        .arg("2")
        .arg("-2")
        .arg("2")
        .succeeds().stdout_only_fixture("different_fields.expected");
}

#[test]
fn different_field() {
    new_ucmd!()
        .arg("fields_2.txt")
        .arg("fields_3.txt")
        .arg("-2")
        .arg("2")
        .succeeds().stdout_only_fixture("different_field.expected");
}

#[test]
fn unpaired_lines() {
    new_ucmd!()
        .arg("fields_2.txt")
        .arg("fields_3.txt")
        .arg("-a")
        .arg("1")
        .succeeds().stdout_only_fixture("fields_2.txt");

    new_ucmd!()
        .arg("fields_3.txt")
        .arg("fields_2.txt")
        .arg("-1")
        .arg("2")
        .arg("-a")
        .arg("2")
        .succeeds().stdout_only_fixture("unpaired_lines.expected");
}

#[test]
fn case_insensitive() {
    new_ucmd!()
        .arg("capitalized.txt")
        .arg("fields_3.txt")
        .arg("-i")
        .succeeds().stdout_only_fixture("case_insensitive.expected");
}

#[test]
fn semicolon_separated() {
    new_ucmd!()
        .arg("semicolon_fields_1.txt")
        .arg("semicolon_fields_2.txt")
        .arg("-t")
        .arg(";")
        .succeeds().stdout_only_fixture("semicolon_separated.expected");
}

#[test]
fn new_line_separated() {
    new_ucmd!()
        .arg("-")
        .arg("fields_2.txt")
        .arg("-t")
        .arg("")
        .pipe_in("1 a\n1 b\n8 h\n")
        .succeeds().stdout_only("1 a\n8 h");
}

#[test]
fn multitab_character() {
    new_ucmd!()
        .arg("semicolon_fields_1.txt")
        .arg("semicolon_fields_2.txt")
        .arg("-t")
        .arg("э")
        .fails().stderr_is("join: error: multi-character tab э");
}

#[test]
fn default_format() {
    new_ucmd!()
        .arg("fields_1.txt")
        .arg("fields_2.txt")
        .arg("-o")
        .arg("1.1 2.2")
        .succeeds().stdout_only_fixture("default.expected");

    new_ucmd!()
        .arg("fields_1.txt")
        .arg("fields_2.txt")
        .arg("-o")
        .arg("0 2.2")
        .succeeds().stdout_only_fixture("default.expected");
}

#[test]
fn unpaired_lines_format() {
    new_ucmd!()
        .arg("fields_2.txt")
        .arg("fields_3.txt")
        .arg("-a")
        .arg("2")
        .arg("-o")
        .arg("1.2 1.1 2.4 2.3 2.2 0")
        .succeeds().stdout_only_fixture("unpaired_lines_format.expected");
}

#[test]
fn autoformat() {
    new_ucmd!()
        .arg("fields_2.txt")
        .arg("different_lengths.txt")
        .arg("-o")
        .arg("auto")
        .succeeds().stdout_only_fixture("autoformat.expected");
}

#[test]
fn empty_format() {
    new_ucmd!()
        .arg("fields_1.txt")
        .arg("fields_2.txt")
        .arg("-o")
        .arg("")
        .fails().stderr_is("join: error: invalid file number in field spec: ''");
}

#[test]
fn empty_key() {
    new_ucmd!()
        .arg("fields_1.txt")
        .arg("empty.txt")
        .arg("-j")
        .arg("2")
        .arg("-a")
        .arg("1")
        .arg("-e")
        .arg("x")
        .succeeds().stdout_only_fixture("empty_key.expected");
}

#[test]
fn missing_format_fields() {
    new_ucmd!()
        .arg("fields_2.txt")
        .arg("different_lengths.txt")
        .arg("-o")
        .arg("0 1.2 2.4")
        .arg("-e")
        .arg("x")
        .succeeds().stdout_only_fixture("missing_format_fields.expected");
}