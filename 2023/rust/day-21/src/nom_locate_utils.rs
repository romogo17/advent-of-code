use glam::IVec2;
use nom_locate::LocatedSpan;

pub type Span<'a> = LocatedSpan<&'a str>;
pub type SpanIVec2<'a> = LocatedSpan<&'a str, IVec2>;

pub fn with_xy(span: Span) -> SpanIVec2 {
    // column and location line are 1-indexed
    let x = span.get_column() as i32 - 1;
    let y = span.location_line() as i32 - 1;
    span.map_extra(|_| IVec2::new(x, y))
}
