# How the plugin works (v0)
- Plugin reads template.md
    - This is just a list of checkboxes that will be copied into the "Daily" section of today's MD file
    - Expects all checkboxes to be unchecked
- Plugin reads the most recent daily MD from the "daily" directory
- Plugin re-outputs most recent daily MD with all incomplete checkboxes removed (to move to today's MD)
- Plugin creates a markdown of the form:
```
<date, ideally human-readable>
# Daily
<list of checkboxes>
<uncompleted contents of previous daily MD>
```
- The "uncompleted contents of previous daily MD" (as described above) will include headings, and the checkboxes will be processed as follows:
    - For any given checkbox, it is only considered complete if all sub-checkboxes (indented) are also complete
        - that applies recursively
        - that also applies if the parent checkbox was (accidentally) marked as complete (checkbox should be copied to the new day as incomplete)

# Post-v0 TODO
- [x] Better error handling and use of "?"
- [ ] Allow user to set a directory other than "daily"
- [ ] Restructure `expect`s into `obsidian::Notice::new` with no error
