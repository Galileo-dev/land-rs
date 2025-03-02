// Common styles for front page
#let front-page-style(body) = {
  set page(numbering: none, margin: (left: 2.5cm, right: 2.5cm, top: 2.5cm, bottom: 2.5cm))
  counter(page).update(1)
  set heading(numbering: none)
  show heading.where(level: 1): it => {
    it
    v(8%, weak: true)
  }
  body
}

// Common styles for main pages
#let main-page-style(body) = {
  set text(features: ("pnum", "liga"))

  set page(
    numbering: "1",
    margin: (left: 2.5cm, right: 2.5cm, top: 2.5cm, bottom: 2.5cm),
    footer: context {
      align(center, text(fill: rgb(150, 150, 150), counter(page).display()))
    },
  )

  counter(page).update(1)
  counter(heading).update(0)

  set heading(numbering: none)

  show heading.where(level: 1): it => {
    set text(weight: "medium", size: 24pt, fill: rgb(50, 50, 50))
    v(1em)
    it
    v(1.5em)
  }

  show heading.where(level: 2): it => {
    set text(weight: "medium", size: 18pt, fill: rgb(50, 50, 50))
    v(1em)
    it
    v(0.8em)
  }

  show heading.where(level: 3): it => {
    set text(weight: "medium", size: 14pt, fill: rgb(50, 50, 50))
    v(0.8em)
    it
    v(0.5em)
  }

  set par(justify: false, leading: 0.65em, first-line-indent: 0pt)
  show par: set block(spacing: 0.65em)

  // Link styling
  show link: set text(fill: rgb(0, 102, 204))

  body
}

// Common styles for back page
#let back-page-style(body) = {
  set heading(numbering: none)
  counter(heading.where(level: 1)).update(0)
  counter(heading).update(0)
  body
}
