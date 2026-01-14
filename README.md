# Scaffy
A convenient project template scaffolding TUI.

## Installation
> If you are on Linux, you might need to install some system dependencies of libraries that are used: https://github.com/http-rs/surf#installation

```
cargo install scaffy
```

## Usage
Just run `scaffy` in any directory, you can choose any template and where to clone it within the TUI. Keyboard navigation instructions are provided at the bottom of the TUI.
### Searchbar
The searchbar considers strictly spelled case-insensitive queries separated by spaces, which are each filtered through every template to see if their name, description, or tags contain it.

## Making a Template

### 1. Add information to `/templates/templates.json`
Every template should be a subdirectory of `/templates` and have a corresponding entry within `templates/templates.json`.
`templates.json` contains an array of objects with the following entries (all required):
<table>
    <thead>
        <th>Key</th>
        <th>Value Type</th>
        <th>Value Description</th>
    </thead>
    <tr>
        <td>name</td>
        <td>string</td>
        <td>A display name; serves as the header for the TUI list entry</td>
    </tr>
    <tr>
        <td>path</td>
        <td>string</td>
        <td>The name of the template folder in <code>/templates</code>. This should be unique from any other template. Do not include a leading slash. </td>
    </tr>
    <tr>
        <td>author</td>
        <td>string</td>
        <td>The author's Github username, preferably</td>
    </tr>
    <tr>
        <td>description</td>
        <td>string</td>
        <td>A concise description about the template's contents. Try to make this less than 100 characters, as extraneous characters are cut off in the TUI.</td>
    </tr>
    <tr>
        <td>tags</td>
        <td>

```ts
Record<
    "languages" | "frameworks" | "libraries" | "misc",
    Record<string, string | null>
>
``` 

</td>
        <td>
            This holds the tags associated with the template. Each tag category holds a record instead of an array in order to have an associated version string, with null being an unspecified vesion.
        </td>
    </tr>
</table>

### 2. Make Template
Add the files/folders of the template to `/templates/<path specified in templates.json>`. Make sure that when you're done, there shouldn't be any unnecessary files (such as node_modules or package_lock.json for node.js, but they should be in the .gitignore).

#### Project Info Replacement Strings
The user can provide a project name in the initialization stage, which can be used in any template file as needed. Just insert the following strings at where the project name should be:

<details>
<summary>Replacement strings chart</summary>
<table>
<thead>
<th>Replacement String</th>
<th>Resulting Inserted Project Name (assuming user entered <code>Project name example</code>)</th>
</thead>
<tr>
<td>@@SCAFFY_PROJECT_NAME@@</td>
<td>Project name example</td>
</tr>
<tr>
<td>@@SCAFFY_PROJECT_NAME_TITLECASE@@</td>
<td>Project Name Example</td>
</tr>
<tr>
<td>@@SCAFFY_PROJECT_NAME_UPPERCASE@@</td>
<td>PROJECT NAME EXAMPLE</td>
</tr>
<tr>
<td>@@SCAFFY_PROJECT_NAME_LOWERCASE@@</td>
<td>project name example</td>
</tr>
<tr>
<td>@@SCAFFY_PROJECT_NAME_SNAKECASE@@</td>
<td>Project_name_example</td>
</tr>
<td>@@SCAFFY_PROJECT_NAME_LOWERSNAKECASE@@</td>
<td>project_name_example</td>
</tr>
<tr>
<tr>
<td>@@SCAFFY_PROJECT_NAME_UPPERSNAKECASE@@</td>
<td>PROJECT_NAME_EXAMPLE</td>
</tr>
<td>@@SCAFFY_PROJECT_NAME_LOWERCAMELCASE@@</td>
<td>projectNameExample</td>
</tr>
<td>@@SCAFFY_PROJECT_NAME_UPPERCAMELCASE@@</td>
<td>ProjectNameExample</td>
</tr>
<tr>
<td>@@SCAFFY_PROJECT_NAME_KEBABCASE@@</td>
<td>Project-name-example</td>
</tr>
<tr>
<td>@@SCAFFY_PROJECT_NAME_LOWERKEBABCASE@@</td>
<td>project-name-example</td>
</tr>
<tr>
<td>@@SCAFFY_PROJECT_NAME_UPPERKEBABCASE@@</td>
<td>PROJECT-NAME-EXAMPLE</td>
</tr>

</table>

</details>

### 3. Generate associated files
After making a template, run `node scripts/gen-paths.js` in order to generate an associated file in `templates/__scaffy_template_contents`. This file is currently neccesary for each template, but this requirement may eventually be unneccesary.