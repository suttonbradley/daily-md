# How the plugin works (v0)
- Plugin uses the `daily` directory in the root of your vault for the location of all files
- Plugin reads every-day.md from the `daily` directory
    - This is content that will be copied into the "Daily" section of the generated MD file. Can be checkboxes, bullets, or anything else
    - Copies as-is, so it expects all checkboxes to be unchecked
- Plugin reads the most recent daily MD from the `daily` directory
- [TODO?] Plugin re-outputs most recent daily MD with all incomplete checkboxes removed (to move to today's MD)
- Plugin creates a markdown of the form:
```
<date, ideally human-readable>
# Daily
<list of checkboxes>
<uncompleted contents of previous daily MD>
```
- The "uncompleted contents of previous daily MD" (as described above) will include headings, and the checkboxes will be processed as follows:
    - For simplicity in the initial version, a checkbox is considered complete only if it is checked, **and** if it is checked, all sub-items are ignored even if not explicitly checked.
        - The following checkbox subtree would be ignored **entirely** in the carryover process because the parent checkbox is completed:
            ``` 
            - [x] todo
                - make sure to do this
                - [ ] sub task that was not completed
                - [x] sub task that was completed
            ```
        - The scenario below would be more ideal. Working on getting around to it.
    - ~~For any given checkbox, it is only considered complete if all sub-checkboxes (indented) are also complete~~
        - ~~that applies recursively~~
        - ~~that also applies if the parent checkbox was (accidentally) marked as complete (checkbox should be copied to the new day as incomplete)~~

# Post-v0 TODO
- [x] Better error handling and use of "?"
- [ ] Header with human-readable date
- [ ] Optimization: get tab count on TodoLine _before_ pulling its contents to see if it's even worthwhile to do so
    - A completed checkbox at a lesser tab count means we don't care about this content -- we're leaving it in the old file
- [ ] Allow user to set a directory other than `daily`
- [ ] Restructure `expect`s into `obsidian::Notice::new` with no error
