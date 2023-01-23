use log::trace; // TODO remove

const CHECKBOX_COMPLETE: &str = "- [x] ";
const CHECKBOX_INCOMPLETE: &str = "- [ ] ";
pub const DAILY_LINE: &str = "# Daily";

// pub struct TodoTree {
//     this: TodoLine,
//     children: Vec<TodoTree>,
// }

#[derive(Debug)]
pub struct TodoLine {
    // Only copy and store the contents if the checkbox is incomplete
    contents: Option<String>,
    tabs: usize,
    is_checkbox: bool,
}

impl TodoLine {
    fn complete(&self) -> bool {
        self.contents.is_none()
    }

    pub fn from_file_contents(contents: String) -> Vec<Self> {
        // Stores (min) number of tabs of a completed parent checkbox of the current line
        let mut completed_parent = None;

        // Advance until we're past the "# Daily" section
        let mut lines = contents.split('\n').peekable();
        if let Some(first_line) = lines.peek() {
            // If first line is "# Daily", move past and iterate until another header
            if *first_line == DAILY_LINE {
                lines.next();
                trace!("About to loop to advance past \"# Daily\" section...");
                while let Some(line) = lines.peek() {
                    trace!("Peeking line:\n{line}");
                    if line.starts_with('#') {
                        break;
                    }
                    lines.next();
                }
            }
        }

        let mut result = vec![];
        for line in lines {
            let todo_line = TodoLine::from(line);
            trace!("todo_line is:\n{todo_line:?}");

            // If this is an incomplete checkbox at <= completed_parent level,
            //     add to vec and make completed_parent None.
            // If this is an incomplete checkbox at > completed_parent level, noop.
            // That should not happen, but we'll consider it complete due to parent completion.
            // If this is a complete checkbox, set completed_parent if not set.

            // Map completed_parent to usize::MAX if None to always pass the check.
            if !todo_line.complete() && todo_line.tabs <= completed_parent.unwrap_or(usize::MAX) {
                result.push(todo_line);
                completed_parent = None;
            } else if todo_line.complete() {
                if let None = completed_parent {
                    completed_parent = Some(todo_line.tabs);
                }
            }
        }
        trace!("result:\n{result:?}");
        result
    }
}

impl ToString for TodoLine {
    fn to_string(&self) -> String {
        // Indent
        let mut result = "\t".repeat(self.tabs).to_string();
        // Replace checkbox (previously consumed by parser)
        if self.is_checkbox {
            result.push_str(CHECKBOX_INCOMPLETE);
        }

        // TODO this is not great -- relying on an unwrap here
        //   implies that the design should have been different.
        // We should only have lines _with_ content calling this function.
        result.push_str(self.contents.clone().unwrap().as_str());
        result
    }
}

impl From<&str> for TodoLine {
    fn from(s: &str) -> Self {
        use nom::{
            bytes::complete::{tag, tag_no_case},
            character::complete::tab,
            Parser,
        };

        let mut s = s;
        // Parse number of tabs preceding this checkbox
        let mut tabs = 0;
        while let Ok((rest, _)) = tab::<&str, ()>.parse(s) {
            tabs += 1;
            s = rest;
        }

        // See if this line matches on a checkbox (complete or incomplete)
        let (is_checkbox, contents) =
            if let Ok((rest, _)) = tag::<_, _, ()>(CHECKBOX_INCOMPLETE).parse(s) {
                // Is a checkbox, incomplete so keep content
                (true, Some(String::from(rest)))
            } else if let Ok((_, _)) = tag_no_case::<_, _, ()>(CHECKBOX_COMPLETE).parse(s) {
                // Is a checkbox, complete so throw away content
                (true, None)
            } else {
                // Not a checkbox, so keep content
                (false, Some(String::from(s)))
            };

        Self {
            tabs,
            contents,
            is_checkbox,
        }
    }
}
