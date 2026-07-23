// Stellar compatibility layer for Typst
// Target: Typst 0.15.x
// Faithful port of stellar.sty (2023/01/01)

#import "@preview/latex-lookalike:0.1.4"

// ---------- Document setup ----------

#let annotation-color = rgb("0000a0")

// Apply with: #show: stellar.with(...)
#let stellar(
  width: 6.5in,
  margin: 0pt,
  font-size: 10pt,
  body,
) = {
  set document(title: "Stellar")

  // `standalone[preview]` crops the output to one content-sized page.  The
  // original `fullpage` setup has a 6.5in text width, so we keep that width
  // fixed while allowing the page to grow vertically without pagination.
  set page(width: width, height: auto, margin: margin)
  show pagebreak: none

  // Do not name a font here. `latex-lookalike` supplies the LaTeX-like style,
  // while Typst uses the fonts available in the caller's installation without
  // emitting a missing-font warning from this module.
  set text(size: font-size)
  set par(first-line-indent: 0pt, spacing: 0.5em + 2pt)
  show link: set text(fill: annotation-color)
  show: latex-lookalike.style.with(numbering: none)
  body
}

// ---------- Stellar command transport ----------

// Emit a directive exactly as one unbreakable PDF text line. The directive is
// typeset in a natural-width inline box and overlaid with `place`, while a
// separate fixed-height block reserves its vertical position. No zero-width
// container participates in laying out the text, because that would force a
// wrap opportunity at every word.
//
// `_padcmd` models the original three-line construction: one blank baseline,
// the directive baseline, and one blank baseline.  Snippet openings get an
// additional 0.35cm after-space to compensate for the negative top offset of
// the colored box and keep the visible gap equivalent to a full blank line.
#let _command-baseline = 12pt

#let _blank-command-line() = block(
  width: 100%,
  height: _command-baseline,
  above: 0pt,
  below: 0pt,
  breakable: false,
)[]

// Typst always lays text out against some finite available width. To emulate
// LaTeX's `\makebox[0pt][l]{...}`, give the directive an explicit composition
// width that grows with its character count, then overlay it with `place` so
// that this very wide box contributes zero horizontal width to normal layout.
// Stellar directives are ASCII/JSON-like, so two em per character is a conservative
// overestimate of their rendered width and prevents every possible soft wrap.
#let _command-composition-width(command) = {
  let chars = str(command).len()
  calc.max(2em, (chars + 1) * 2em)
}

#let _command-content(command) = {
  set text(hyphenate: false)
  set par(justify: false)
  str(command)
}

#let _command-line(command) = block(
  width: 100%,
  height: _command-baseline,
  above: 0pt,
  below: 0pt,
  breakable: false,
  clip: false,
)[
  // The placed box may extend arbitrarily far beyond the PDF's right edge.
  // Its explicit over-wide layout area prevents wrapping, while `place` keeps
  // that width out of the document flow, matching a zero-width LaTeX makebox.
  #place(
    top + left,
    box(
      width: _command-composition-width(command),
      clip: false,
      _command-content(command),
    ),
  )
]

#let _padcmd(command, before: _command-baseline, after: _command-baseline) = block(
  width: 100%,
  above: 0pt,
  below: 0pt,
  breakable: false,
)[
  #block(width: 100%, height: before, above: 0pt, below: 0pt)[]
  #_command-line(command)
  #block(width: 100%, height: after, above: 0pt, below: 0pt)[]
]

#let stellar-section(title) = _padcmd("!section " + str(title))
#let stellar-subsection(title) = _padcmd("!subsection " + str(title))
#let stellar-subsubsection(title) = _padcmd("!subsubsection " + str(title))
#let genpage() = _padcmd("!gen-page true")
#let id(value) = _padcmd("!id " + str(value))

#let snippetref(identifier, body: none) = {
  let shown = if body == none { str(identifier) } else { body }
  link("/snippet/" + str(identifier), shown)
}

#let labelref(destination, body) = link("|" + str(destination), body)

// Collapses source newlines in plain HTML to spaces, like the verbatim-based
// LaTeX environment.
#let plainhtml(body) = {
  let value = if type(body) == str { body } else { repr(body) }
  let collapsed = value.replace(regex("\\s*\\n\\s*"), " ")
  _padcmd("!plain " + collapsed)
}

#let includesnpt(identifier, params: (:)) = {
  let encoded = if type(params) == dictionary {
    params.pairs().map(pair => str(pair.first()) + "=" + str(pair.last())).join("&")
  } else if type(params) == array {
    params.map(str).join("&")
  } else {
    str(params)
  }
  let suffix = if encoded == "" { "" } else { " " + encoded }
  _padcmd("!include " + str(identifier) + suffix)
}

// ---------- Metadata JSON fragments ----------

#let _json-escape(value) = {
  let escaped = str(value)
  escaped = escaped.replace("\\", "\\\\")
  escaped = escaped.replace("\"", "\\\"")
  escaped = escaped.replace("\n", "\\n")
  escaped = escaped.replace("\r", "\\r")
  escaped = escaped.replace("\t", "\\t")
  escaped
}

#let metastring(key, value) = (
  "{ \"" + _json-escape(key) + "\" : \"" + _json-escape(value) + "\" } "
)

#let metanumber(key, value) = (
  "{ \"" + _json-escape(key) + "\" : " + str(value) + " } "
)

#let metabool(key, value) = {
  let scalar = if value { "true" } else { "false" }
  "{ \"" + _json-escape(key) + "\" : " + scalar + " } "
}

#let metalist(key, values) = {
  let items = if type(values) == array { values } else { (values,) }
  let encoded = items.map(value => "\"" + _json-escape(value) + "\"").join(",")
  "{ \"" + _json-escape(key) + "\" : [" + encoded + "] } "
}

#let metajson(value) = str(value) + " "
#let generalizations(values) = metalist("generalizations", values)

#let snippet(identifier, body, metadata: "") = {
  let suffix = if str(metadata) == "" { "" } else { " " + str(metadata) }
  _padcmd(
    "!snippet " + str(identifier) + suffix,
    after: _command-baseline + 0.35cm,
  )
  body
  _padcmd("!endsnippet")
}

// ---------- Colored sentence boxes ----------

#let theorem-bg = rgb("f2f2f9")
#let theorem-fr = rgb("00007b")
#let lemma-bg = rgb("fffaf8")
#let lemma-fr = rgb("983b0f")
#let proposition-bg = rgb("f2fbfc")
#let proposition-fr = rgb("191971")
#let corollary-bg = rgb("f9effb")
#let corollary-fr = rgb("a74eb4")
#let definition-bg = rgb("fff2f2")
#let definition-fr = rgb("cc0000")
#let example-bg = rgb("f2fbf8")
#let example-fr = rgb("2e8181")
#let note-bg = rgb("e6e6e6")
#let note-fr = rgb("6f6f6f")
#let proof-bg = rgb("d9ffe3")
#let proof-fr = rgb("32a852")
#let exercise-bg = rgb("f9effb")
#let exercise-fr = rgb("a74eb4")
#let solution-bg = rgb("fffaf8")
#let solution-fr = rgb("983b0f")
#let character-bg = rgb("d5d9e0")
#let character-fr = rgb("2a3547")
#let summary-bg = rgb("f2f2f9")
#let summary-fr = rgb("00007b")

#let _sentence-box(kind, title, background, frame, body) = block(
  width: 100%,
  breakable: true,
  fill: background,
  stroke: (left: 2pt + frame),
  radius: 0pt,
  inset: (left: 10pt, right: 10pt, top: 8pt, bottom: 8pt),
  above: -0.35cm,
  below: 0pt,
)[
  #set par(first-line-indent: 0pt)
  #text(weight: "bold", fill: frame)[#kind]
  #if title != none and title != [] [
    #h(0.45em)#text(weight: "regular", fill: frame)[#title]
  ]
  #parbreak()
  #v(0.25em)
  #body
]

#let stheorem(title, body) = _sentence-box([Theorem], title, theorem-bg, theorem-fr, body)
#let scorollary(title, body) = _sentence-box([Corollary], title, corollary-bg, corollary-fr, body)
#let slemma(title, body) = _sentence-box([Lemma], title, lemma-bg, lemma-fr, body)
#let sproposition(title, body) = _sentence-box([Proposition], title, proposition-bg, proposition-fr, body)
#let sdefinition(title, body) = _sentence-box([Definition], title, definition-bg, definition-fr, body)
#let sexample(title, body) = _sentence-box([Example], title, example-bg, example-fr, body)
#let snote(title, body) = _sentence-box([Note], title, note-bg, note-fr, body)
#let saxiom(title, body) = _sentence-box([Axiom], title, definition-bg, definition-fr, body)
#let sproof(title, body) = _sentence-box([Proof], title, proof-bg, proof-fr, body)
#let sexercise(title, body) = _sentence-box([Exercise], title, exercise-bg, exercise-fr, body)
#let ssolution(title, body) = _sentence-box([Solution], title, solution-bg, solution-fr, body)
#let scharacter(title, body) = _sentence-box([Character], title, character-bg, character-fr, body)
#let ssummary(title, body) = _sentence-box([Summary], title, summary-bg, summary-fr, body)

#let qed-symbol = labelref("Quod erat demonstrandum!", text(fill: black)[■])
#let qed() = block(width: 100%, above: 0pt, below: 0pt)[#h(1fr)#qed-symbol]

// ---------- Combined snippet environments ----------

#let snippettheorem(identifier, title, metadata: "", body) = snippet(
  identifier,
  stheorem(title, body),
  metadata: metastring("type", "Theorem") + str(metadata),
)

#let snippetcorollary(identifier, title, metadata: "", body) = snippet(
  identifier,
  scorollary(title, body),
  metadata: metastring("type", "Corollary") + str(metadata),
)

#let snippetlemma(identifier, title, metadata: "", body) = snippet(
  identifier,
  slemma(title, body),
  metadata: metastring("type", "Lemma") + str(metadata),
)

#let snippetproposition(identifier, title, metadata: "", body) = snippet(
  identifier,
  sproposition(title, body),
  metadata: metastring("type", "Proposition") + str(metadata),
)

#let snippetdefinition(identifier, title, metadata: "", body) = snippet(
  identifier,
  sdefinition(title, body),
  metadata: metastring("type", "Definition") + str(metadata),
)

#let snippetexample(identifier, title, metadata: "", body) = snippet(
  identifier,
  sexample(title, body),
  metadata: metastring("type", "Example") + str(metadata),
)

#let snippetnote(identifier, title, metadata: "", body) = snippet(
  identifier,
  snote(title, body),
  metadata: metastring("type", "Note") + str(metadata),
)

#let snippetaxiom(identifier, title, metadata: "", body) = snippet(
  identifier,
  saxiom(title, body),
  metadata: metastring("type", "Axiom") + str(metadata),
)

#let snippetproof(identifier, proves, title, metadata: "", body) = snippet(
  identifier,
  sproof(snippetref(proves, body: text(fill: proof-fr)[#title]), [#body #qed()]),
  metadata: metastring("type", "Proof") + metastring("proves", proves) + str(metadata),
)

#let snippetexercise(identifier, title, metadata: "", body) = snippet(
  identifier,
  sexercise(title, body),
  metadata: metastring("type", "Exercise") + str(metadata),
)

#let snippetsolution(identifier, title, metadata: "", body) = snippet(
  identifier,
  ssolution(title, body),
  metadata: metastring("type", "Solution") + str(metadata),
)

#let snippetcharacter(identifier, title, metadata: "", body) = snippet(
  identifier,
  scharacter(title, body),
  metadata: metastring("type", "Character") + str(metadata),
)

#let snippetsummary(identifier, title, metadata: "", body) = snippet(
  identifier,
  ssummary(title, body),
  metadata: metastring("type", "Summary") + str(metadata),
)
