WIP

Generate markdown footer links.

### Example:

```markdown
- [md-footer](https://github.com/sayanarijit/md-footer)
- `[ignoreme](https://link.to.ignore)`

[![xplr](https://s3.gifyu.com/images/xplr-0.5.0.gif)](https://github.com/sayanarijit/xplr)
```

Will convert into

```markdown
- [md-footer][1]
- `[ignoreme](https://link.to.ignore)`

[![xplr][2]][3]


[1]:https://github.com/sayanarijit/md-footer
[2]:https://s3.gifyu.com/images/xplr-0.5.0.gif
[3]:https://github.com/sayanarijit/xplr
```

### Usage:

```bash
cat /path/fo/file.md | md-footer

# or

md-footer /path/to/footer.md
```

It was used to [generate footer links for the xplr book](https://github.com/sayanarijit/xplr/pull/294/files).
