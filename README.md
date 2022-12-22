# dg

**README Sections:** [Presentation](#presentation) - [Usage](#usage) - [Data Definitions](#data) - [Installation](#installation) - [Contributing](#contributing)

---

**dg** (for data generator) is a general purpose lightweight mock document generator 
(useful for generating big payloads for software test suites) that aims to be very simple (yet flexible).

---

<a id="presentation">
<h1>Presentation</h1>
</a>

Dg aims to create big payload documents with the same randomized data applied to one or many 
<a href="https://handlebarsjs.com/">handlebars</a> templates. 
This project is rooted in the need to fastly and easily generate mock data in the context of a CI/CD 
environment.
Dg uses a list of items which are randomly generated to be used in a handlebars template.
These items come with a small list of primitives that render data directly, and can be completed
with custom data definitions (either as stacked options or in a file).

<a id="usage">
<h1>Usage</h1>
</a>

### Arguments
Dg uses a list of templates as input arguments. 
Without specifying output files, all the templates are rendered sequentially
on the standard output.

### Options
- **-c**, **--count** : The number of items to generate (defaults to 10)
- **-p**, **--prefix**: by defaults, the item list is named ```item```, this can be changed with this option
- **-o**, **--output**: Can be used several times (typically matching the number of templates we're rendering).
It will be associated sequencially with the templates passed as arguments. If the number of output files is 
less than the number of templates, the remaining templates will be rendered on stdout.
- **-r**, **--random-sgtring-size**: One of the primitives datagen is yielding in its items is a random string.
This option is here to setup the length of it (defaults to 10 characters).
- **-d**, **--definitions**: Stackable data definitions (see section below).
- **--definitions-file**: Data definitions file (see section below).
- **-h**, **--help**: Print the help
- **-V**, **--version**: Print the version

### Examples
- `dg -c 1000 --definitions-file ./definitions_1 -o ./1000-units.xml ./units.template.xml`: Will use the template `units.template.xml` to generate the file `1000-units.xml` with extra data definitions from `definitions_1` and containing a rendered 1000 items.
- `dg -o ./10-units.xml ./units.template.xml ./units.template.json`: Will render 10 elements in `10-units.xml` from template `units.template.xml` and then render 10 elements from `units.template.json` on stdout
- `dg -o ./10-units.xml -o ./10-units.json ./units.template.xml ./units.template.json` is similar to the line above except that the json template is rendered in `10-units.json`

### Template Example

```xml
<body>
    <some>value</some>
    {{#each item}}
    <foo>
        <random>{{this.random}}</random>
        <randomstring>{{this.randstr}}</randomstring>
        <increment>{{this.increment}}</increment>
        <custom>{{this.custom}}</custom>
        <small>{{this.small}}</small>
        <person>{{this.firstname}} {{this.lastname}}</person>
    </foo>
    {{/each}}
</body>
```

Here the items are stored in `item`.
The primitives `random`, `randstr`, `increment`, `firstname` and `lastname` are used.
The others are custom data defintions. We will cover that below.

<a id="data">
<h1>Data Definitions</h1>
</a>

### Primitives

Dg yields some primitives in each item it generates: 
- **random**: is a signed random integer
- **randstr**: a fixed-length random string
- **increment**: the index of the item
- **firstname**: a random but correct first name
- **lastname**: a random but credible last name

The idea is to add more of them everytime a use case is spotted by someone.

### Extra Data Definitions

The **-d** and **--definitions-file** options work in a very similar ways. The main difference is that the first one
inlines one or several custom data definitions while the other sets a file path holding definitions to be fully parsed.
In the file or inlined, the definitions come in the form: `key = expression`.
The following arithmetic operations/rules are supported :
- \+
- \-
- \*
- /
- %
- parentheses and precedence

Also here, in order to have a full purpose for it, there are two primitives that are available in the form of
function-like calls:
- incr(): Returns the current value of the item's index
- random(low:high): generates a random value between low and high (where low and high are integers).

### Example

With the template

```xml
{{#each item}}
<custom>{{this.custom}}</custom>
{{/each}}
```

And with the command `dg -c 5 -d "custom = incr() % 3" ./template.xml`,
We will generate the following

```xml
<custom>0<custom>
<custom>1<custom>
<custom>2<custom>
<custom>0<custom>
<custom>1<custom>
```

### Data Definition Files

Very simple: every `key = expression` item is separated by a new line.
So you can stack them in the following way

```
foo = incr() % 5
bar = random(2:5)
baz = incr() / 2
```

NB : Be careful of dividing by zero !

<a id="installation">
<h1>Installation</h1>
</a>

For now, it is only possible to install it via the <a href="https://rustup.rs/">rust</a> toolchain
(follow the link to install it :)).
When you're done installing rust:
- `git clone https://github.com/aitrs/dg `
- `cd dg`
- `cargo install --path .`

And then you will have it in your path.

<a id="contributing">
<h1>Contributing</h1>
</a>

I'm aware that this project is still something that fullfills a very specific need I had at one point.
I'm also aware that data generators still exist in a really much more advanced way, but I really wanted something
very specific. Everyone who wants to contribute is welcome here. Just fork it and send me a PR when you're done adding 
new features. For now this tool is really simple so there is no need to add complicated dev pipelines...

There are some todos : 
- Add unit tests
- Add more primitive generators (phone numbers, realistic addresses, everything that comes to mind)
- Extend the definitions parser to be able to have a little interpreted language to manipulate the data structures at the sub-item level.