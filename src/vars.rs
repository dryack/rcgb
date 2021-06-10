/*The MIT License (MIT)

Copyright (c) 2021 David Ryack

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use iota::iota;

pub(crate) const KIB: f64         = 1024.0;
pub(crate) const MIB: f64         = 1048576.0;
pub(crate) const GIB: f64         = 1073742000.0;
pub(crate) const TIB: f64         = 1099512000000.0;
pub(crate) const PROG_VER: &str     = "rgb 0.15";
pub(crate) const AUTH_ADDY: &str    = "<git.lamashtu@gmail.com>";
pub(crate) const LICENSE_TEXT: &str = "The MIT License (MIT)\nCopyright (c) 2021 David Ryack\n\nPermission is hereby granted, free of \
charge, to any person obtaining a copy of\nthis software and associated documentation files (the \"Software\"), \
 to deal in\nthe Software without restriction, including without limitation the rights to\nuse, copy, \
modify, merge, publish, distribute, sublicense, and/or sell copies of\nthe Software, and to permit persons \
to whom the Software is furnished to do so,\nsubject to the following conditions:\n\nThe above copyright \
notice and this permission notice shall be included in all\ncopies or substantial portions of the Software. \
\n \nTHE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR\nIMPLIED, INCLUDING BUT \
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS\nFOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN \
 NO EVENT SHALL THE AUTHORS OR\nCOPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, \
WHETHER\nIN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN\nCONNECTION WITH THE \
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.\"";
pub(crate) const MAX_PRECISION: u8 = 9                                           ;

iota! {
      pub const T: u16 = 1 << iota;
      , G
      , M
      , K
      , TG
      , TM
      , TK
      , GM
      , GK
      , MK
      , TGM
      , TMK
      , TGK
      , GMK
      , TGMK
}




