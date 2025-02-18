// Common styles for front page
#let front-page-style(body) = {
  set page(numbering: "i")
  counter(page).update(1)
  set heading(numbering: none)
  show heading.where(level: 1): it => {
    it
    v(6%, weak: true)
  }
  body
}

// Common styles for main pages
#let main-page-style(body) = {
  set text(features: ("onum",))
  set page(
    numbering: "1",
    // Only show numbering in footer when no chapter header is present
    footer: context {
      let chapters = heading.where(level: 1)
      if query(chapters).any(it => it.location().page() == here().page()) {
        align(center, counter(page).display())
      } else {
        none
      }
    },
  )
  counter(page).update(1)
  counter(heading).update(0)
  set heading(numbering: "1.1")
  show heading.where(level: 1): it => {
    it
    v(12%, weak: true)
  }
  body
}

// Common styles for back page
#let back-page-style(body) = {
  set heading(numbering: "A.1.1", supplement: [Appendix])
  // Make sure headings start with 'A'
  counter(heading.where(level: 1)).update(0)
  counter(heading).update(0)
  body
}