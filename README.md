[![Crate](https://img.shields.io/crates/v/rc_pdf?style=flat-square)](https://crates.io/crates/rc_pdf)
[![License](https://img.shields.io/crates/l/rc_pdf?style=flat-square)](https://github.com/Jockerkat/rcPDF/blob/master/LICENSE.md)
[![Maintenance](https://img.shields.io/maintenance/yes/2022?style=flat-square)]()

# rcPDF

## About

rcPDF aims to facilitate creating conforming PDFs as a conforming writer as laid out in the *[ISO 32000-1:2008](https://www.adobe.com/content/dam/acom/en/devnet/pdf/pdfs/PDF32000_2008.pdf)* standard.
rcPDF is page-oriented (as defined in *ISO 32000-1:2008*, 4 "Terms and definitions", 4.19 "electronic document"), meaning that all data is processed on a per-page basis.
The aim of the API is to be easy-to-use and intuitive.

The [GitHub repository](https://github.com/Jockerkat/rcPDF) is a mirror of the [GitLab repository](https://gitlab.com/Jockerkat/rcpdf).
If you want to raise an issue or contribute to rcPDF, please do so on GitLab.

**Disclaimer**: Development has only just started, so everything is very much incomplete and subject to change.
Also, I'm new to the rust programming language, so please bear that in mind.

## Table of Contents

- [FAQ](#faq)
- [Changelog](#changelog)
- [License](#license)
- [Credits](#credits)

## FAQ

**Question**: What does "rcPDF" stand for?

**Answer**: rcPDF is short for "rust create PDF" (I'm bad at coming up with names :p).

---

**Question**: Why is development so slow?

**Answer**: Due to the sheer size of the PDF *ISO 32000-1:2008* spec, and life in general (believe me, I want things to move faster, too).

## Changelog

View the changelog [here](CHANGELOG.md).

## License

View the licence [here](LICENCE.md).

## Credits

Ferris, the crab in the repository's icon, is designed by [Karen Rustad Tölva](https://rustacean.net/).
