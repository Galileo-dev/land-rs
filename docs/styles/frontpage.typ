#let frontpage(
  title: [],
  subtitle: "",
  author: "",
  degree: "",
  faculty: "",
  department: "",
  date: none,
) = {
  set document(title: title, author: author)
  set page(
    paper: "a4",
    margin: (left: 2.5cm, right: 2.5cm, top: 2.5cm, bottom: 2.5cm),
    header: none,
    footer: none,
    numbering: none,
  )

  set text(size: 12pt, lang: "en")

  // Logo placement
  place(
    top + center,
    dy: 3cm,
    image("../assets/logo.png", width: 35%),
  )

  // Title
  place(
    top + center,
    dy: 10cm,
    box(
      width: 80%,
      align(center, text(24pt, weight: "medium", fill: rgb(50, 50, 50), title)),
    ),
  )

  // Subtitle
  if subtitle != "" {
    place(
      top + center,
      dy: 12cm,
      box(
        width: 70%,
        align(center, text(14pt, weight: "regular", fill: rgb(80, 80, 80), subtitle)),
      ),
    )
  }

  // Author, department, and other details
  place(
    bottom + center,
    dy: -3cm,
    box(
      width: 80%,
      align(center)[
        #text(12pt, weight: "regular", author)
        #v(0.5em)
        #text(10pt, fill: rgb(100, 100, 100), weight: "light", department)
        #v(0.3em)
        #text(10pt, fill: rgb(100, 100, 100), weight: "light", faculty)
        #v(0.3em)
        #text(
          10pt,
          fill: rgb(100, 100, 100),
          weight: "light",
          degree + " — " + date.display("[month repr:long] [year]"),
        )
      ],
    ),
  )
}
