use std::ops::Deref;
use unicode_width::UnicodeWidthStr;

pub(crate) struct WidthLimited<T, Item, Iter>
where
    T: UnicodeWidthStr + ?Sized,
    Item: Deref<Target = T>,
    Iter: Iterator<Item = Item>,
{
    iter: Iter,
    total_width: usize,
    max_width: usize,
}

impl<T, Item, Iter> WidthLimited<T, Item, Iter>
where
    T: UnicodeWidthStr + ?Sized,
    Item: Deref<Target = T>,
    Iter: Iterator<Item = Item>,
{
    pub(crate) fn new(iter: Iter, max_width: usize) -> Self {
        Self {
            iter,
            total_width: 0,
            max_width,
        }
    }
}

impl<T, Item, Iter> Iterator for WidthLimited<T, Item, Iter>
where
    T: UnicodeWidthStr + ?Sized,
    Item: Deref<Target = T>,
    Iter: Iterator<Item = Item>,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.total_width >= self.max_width {
            return None;
        }

        let item = self.iter.next()?;
        let item_width = item.width();

        if item_width + self.total_width <= self.max_width {
            self.total_width += item_width;
            Some(item)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_width_less_than_max() {
        let input = ["r", "u", "s", "t"].iter().copied();
        let mut width_limited: WidthLimited<str, &str, _> = WidthLimited::new(input, 10);

        assert_eq!(width_limited.next(), Some("r"));
        assert_eq!(width_limited.next(), Some("u"));
        assert_eq!(width_limited.next(), Some("s"));
        assert_eq!(width_limited.next(), Some("t"));
        assert_eq!(width_limited.next(), None);
    }

    #[test]
    fn input_width_equal_to_max() {
        let input = "hello".chars().map(|c| c.to_string());
        let mut width_limited: WidthLimited<str, String, _> = WidthLimited::new(input, 5);

        assert_eq!(width_limited.next(), Some("h".to_string()));
        assert_eq!(width_limited.next(), Some("e".to_string()));
        assert_eq!(width_limited.next(), Some("l".to_string()));
        assert_eq!(width_limited.next(), Some("l".to_string()));
        assert_eq!(width_limited.next(), Some("o".to_string()));
        assert_eq!(width_limited.next(), None);
    }

    #[test]
    fn input_width_greater_than_max() {
        //                     Width: 1 2345 6789
        let input = "a long text".split_whitespace();
        let mut width_limited: WidthLimited<str, &str, _> = WidthLimited::new(input, 8);

        assert_eq!(width_limited.next(), Some("a"));
        assert_eq!(width_limited.next(), Some("long"));
        assert_eq!(width_limited.next(), None);
    }
}
