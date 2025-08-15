# Example: Adding a Custom Template

Here's a complete example of how to add your own custom template to the DTU Notes CLI.

## Step 1: Create Your Template Repository

Create a GitHub repository with the following structure:

```
my-dtu-template/
├── README.md
└── template/
    ├── lib.typ
    └── template.typ
```

### lib.typ

```typst
#let my-custom-note(
  course: "",
  course-name: "",
  title: "",
  date: datetime.today(),
  author: "",
  semester: ""
) = {
  set document(title: title, author: author)
  set page(margin: 2.5cm, numbering: "1")

  // Custom header with your style
  align(center)[
    #rect(
      width: 100%,
      fill: rgb("#1f2937"),
      inset: 1em
    )[
      #text(size: 18pt, weight: "bold", fill: white)[
        #course - #course-name
      ]

      #v(0.5em)

      #text(size: 14pt, fill: white)[
        #title
      ]
    ]

    #v(0.5em)

    #text(size: 11pt)[
      #author | #semester | #date.display("[day]/[month]/[year]")
    ]
  ]

  v(1.5cm)

  // Set up your preferred styling
  set text(size: 11pt)
  set heading(numbering: "1.")

  show heading: it => [
    #v(1em)
    #block(
      fill: rgb("#f3f4f6"),
      width: 100%,
      inset: 0.5em,
      radius: 3pt
    )[
      #it
    ]
    #v(0.5em)
  ]
}

// Custom boxes for different content types
#let important(content) = block(
  fill: rgb("#fef3c7"),
  stroke: (left: 3pt + rgb("#f59e0b")),
  width: 100%,
  inset: 1em
)[
  #text(weight: "bold")[⚠️ Important]

  #content
]

#let example(content) = block(
  fill: rgb("#ecfdf5"),
  stroke: (left: 3pt + rgb("#10b981")),
  width: 100%,
  inset: 1em
)[
  #text(weight: "bold")[💡 Example]

  #content
]

#let note(content) = block(
  fill: rgb("#eff6ff"),
  stroke: (left: 3pt + rgb("#3b82f6")),
  width: 100%,
  inset: 1em
)[
  #text(weight: "bold")[📝 Note]

  #content
]
```

### template.typ

```typst
#import "lib.typ": my-custom-note, important, example, note
```

## Step 2: Add Your Template

```bash
# Add your custom template repository
noter config add-template-repo my-style yourusername/my-dtu-template

# Check that it was added
noter config list-template-repos
```

## Step 3: Download and Test

```bash
# Download your template
noter template update

# Check status
noter template status

# Create a test note
noter note 02101
```

## Step 4: Customize Template Usage

Your template will now be used when creating notes. The generated content will look like:

```typst
#import "@local/dtu-template:0.1.0": *

#show: my-custom-note.with(
  course: "02101",
  course-name: "Introduction to Programming",
  title: "Lecture - December 15, 2024",
  date: datetime.today(),
  author: "Your Name",
  semester: "2024 Fall"
)

= Key Concepts

#important[
  Key takeaways from today's lecture
]

= Mathematical Framework

= Examples

#example[
  Insert example here...
]

= Questions

#note[
  What questions do I have about this topic?
]
```

## Advanced: Multiple Templates

You can have multiple templates for different purposes:

```bash
# Add a minimal template for quick notes
noter config add-template-repo minimal john/minimal-dtu-template

# Add a formal template for assignments
noter config add-template-repo formal mary/formal-dtu-template

# List all templates (first one will be used by default)
noter config list-template-repos

# Disable a template temporarily
noter config enable-template-repo formal false
```

## Template Priority

Templates are used in this order:

1. First enabled custom repository
2. Second enabled custom repository
3. ...
4. Official DTU template (if fallback is enabled)

To change the order, you need to remove and re-add repositories in your preferred order, or edit the config file directly.

## Troubleshooting

If your template doesn't work:

```bash
# Check what's installed
noter template status

# Force reinstall
noter template reinstall

# Check for errors in template files
noter note test-course
```

## Making Templates Available to Others

1. Create a GitHub release in your template repository
2. Share the repository name: `yourusername/my-dtu-template`
3. Others can add it with:
   ```bash
   noter config add-template-repo shared-template yourusername/my-dtu-template
   ```
