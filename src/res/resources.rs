// wengwengweng

const TEMPLATE_2D_VERT: &str = include_str!("2d_template.vert");
const TEMPLATE_2D_FRAG: &str = include_str!("2d_template.frag");

const DEFAULT_2D_VERT: &str = include_str!("2d_default.vert");
const DEFAULT_2D_FRAG: &str = include_str!("2d_default.frag");

const DEFAULT_FONT: &[u8] = include_bytes!("CP437.png");
const DEFAULT_FONT_COLS: usize = 32;
const DEFAULT_FONT_ROWS: usize = 8;
const DEFAULT_FONT_CHARS: &str = r##" ☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼ !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáíóúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╢╖╕╣║╗╝╜╛┐└┴┬├─┼╞╟╚╔╩╦╠═╬╧╨╤╥╙╘╒╓╫╪┘┌█▄▌▐▀αßΓπΣσµτΦΘΩδ∞φε∩≡±≥≤⌠⌡÷≈°∙·√ⁿ²■"##;

