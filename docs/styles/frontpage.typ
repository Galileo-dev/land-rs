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
    margin: (left: 3mm, right: 3mm, top: 12mm, bottom: 27mm),
    header: none,
    footer: none,
    numbering: none,
    number-align: center,
  )

  set text(size: 12pt, lang: "en")

  set par(leading: 0.5em)

  // Faculty
  place(
    top + left,
    dy: 30mm,
    dx: 27mm,
    text(12pt, weight: "light", faculty),
  )

  // Department
  place(
    top + left,
    dy: 36mm,
    dx: 27mm,
    text(12pt, weight: "light", department),
  )

  // Title
  place(
    top + left,
    dy: 43mm,
    dx: 27mm,
    text(14pt, weight: "semibold", title),
  )

  // Subtitle (optional)
  if (subtitle != "") {
    place(
      top + left,
      dy: 49mm,
      dx: 27mm,
      box(
        width: 150mm,
        text(12pt, weight: "light", subtitle),
      ),
    )
  }

  // Author
  place(
    top + left,
    dy: 61mm,
    dx: 27mm,
    text(10pt, weight: "light", author),
  )

  // Description, Degree and Program
  place(
    top + left,
    dy: 67mm,
    dx: 27mm,
    text(
      10pt,
      weight: "light",
      degree + "  — " + date.display("[month repr:long] [year]"),
    ),
  )

  // Image
  place(
    horizon + center,
    dy: 0mm,
    image("../assets/logo.png", width: 60%),
  )
}
