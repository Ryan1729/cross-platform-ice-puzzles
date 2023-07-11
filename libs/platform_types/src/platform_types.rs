/// A colur represented as a byte for each of Alpha, Red, Green, and Blue.
/// Alpha is the most significant byte, blue is the least:
/// `0xaarrggbb`
pub type ARGB = u32;

pub mod unscaled {
    ///! Values are in pixels.

    pub type Inner = u16;

    pub const fn inner_from_u8(byte: u8) -> Inner {
        byte as Inner
    }

    macro_rules! def {
        ($($name: ident, $inner_name: ident)+) => {$(
            pub type $inner_name = Inner;
            #[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub struct $name(pub $inner_name);

            impl $name {
                pub const fn get(self) -> $inner_name {
                    self.0
                }
            }

            impl From<$name> for $inner_name {
                fn from(to_convert: $name) -> $inner_name {
                    $inner_name::from(to_convert.0)
                }
            }
        )*}
    }

    def!{
        X, XInner
        Y, YInner
        W, WInner
        H, HInner
    }

    pub const fn w_to_usize(w: W) -> usize {
        w.0 as usize
    }

    impl From<W> for usize {
        fn from(w: W) -> Self {
            w_to_usize(w)
        }
    }

    pub const fn h_to_usize(h: H) -> usize {
        h.0 as usize
    }

    impl From<H> for usize {
        fn from(h: H) -> Self {
            h_to_usize(h)
        }
    }

    pub const fn w_const_add(a: W, b: W) -> W {
        W(a.0 + b.0)
    }

    pub const fn w_const_sub(a: W, b: W) -> W {
        W(a.0 - b.0)
    }

    pub const fn w_const_mul(a: W, b: Inner) -> W {
        W(a.0 * b)
    }

    pub const fn w_const_div(a: W, b: Inner) -> W {
        W(a.0 / b)
    }

    pub const fn h_const_add(a: H, b: H) -> H {
        H(a.0 + b.0)
    }

    pub const fn h_const_sub(a: H, b: H) -> H {
        H(a.0 - b.0)
    }

    pub const fn h_const_mul(a: H, b: Inner) -> H {
        H(a.0 * b)
    }

    pub const fn h_const_div(a: H, b: Inner) -> H {
        H(a.0 / b)
    }

    pub const fn x_const_add_w(x: X, w: W) -> X {
        X(x.0 + w.0)
    }

    pub const fn y_const_add_h(y: Y, h: H) -> Y {
        Y(y.0 + h.0)
    }

    macro_rules! self_add_sub_def {
        ($($name: ident)+) => {$(
            impl core::ops::AddAssign for $name {
                fn add_assign(&mut self, other: Self) {
                    self.0 += other.0;
                }
            }

            impl core::ops::Add for $name {
                type Output = Self;

                fn add(mut self, other: Self) -> Self::Output {
                    self += other;
                    self
                }
            }

            impl core::ops::SubAssign for $name {
                fn sub_assign(&mut self, other: Self) {
                    self.0 -= other.0;
                }
            }

            impl core::ops::Sub for $name {
                type Output = Self;

                fn sub(mut self, other: Self) -> Self::Output {
                    self -= other;
                    self
                }
            }

            impl $name {
                pub fn saturating_sub(self, other: Self) -> Self {
                    Self(self.0.saturating_sub(other.0))
                }
            }
        )*}
    }

    self_add_sub_def!{W H}

    impl core::ops::AddAssign<W> for X {
        fn add_assign(&mut self, other: W) {
            self.0 += other.0;
        }
    }

    impl core::ops::Add<W> for X {
        type Output = Self;

        fn add(mut self, other: W) -> Self::Output {
            self += other;
            self
        }
    }

    impl core::ops::SubAssign<W> for X {
        fn sub_assign(&mut self, other: W) {
            self.0 -= other.0;
        }
    }

    impl core::ops::Sub<W> for X {
        type Output = Self;

        fn sub(mut self, other: W) -> Self::Output {
            self -= other;
            self
        }
    }

    impl core::ops::Sub<X> for X {
        type Output = W;

        fn sub(self, other: X) -> Self::Output {
            W(self.0 - other.0)
        }
    }

    impl X {
        pub const fn saturating_add(self, w: W) -> X {
            X(self.0.saturating_add(w.0))
        }
        pub const fn saturating_sub(self, w: W) -> X {
            X(self.0.saturating_sub(w.0))
        }
        pub const fn saturating_point_sub(self, x: X) -> W {
            W(self.0.saturating_sub(x.0))
        }
    }

    impl core::ops::AddAssign<H> for Y {
        fn add_assign(&mut self, other: H) {
            self.0 += other.0;
        }
    }

    impl core::ops::Add<H> for Y {
        type Output = Self;

        fn add(mut self, other: H) -> Self::Output {
            self += other;
            self
        }
    }

    impl core::ops::SubAssign<H> for Y {
        fn sub_assign(&mut self, other: H) {
            self.0 -= other.0;
        }
    }

    impl core::ops::Sub<H> for Y {
        type Output = Self;

        fn sub(mut self, other: H) -> Self::Output {
            self -= other;
            self
        }
    }

    impl core::ops::Sub<Y> for Y {
        type Output = H;

        fn sub(self, other: Y) -> Self::Output {
            H(self.0 - other.0)
        }
    }

    impl Y {
        pub const fn saturating_add(self, h: H) -> Y {
            Y(self.0.saturating_add(h.0))
        }
        pub const fn saturating_sub(self, h: H) -> Y {
            Y(self.0.saturating_sub(h.0))
        }
        pub const fn saturating_point_sub(self, y: Y) -> H {
            H(self.0.saturating_sub(y.0))
        }
    }

    #[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
    pub struct XY {
        pub x: X,
        pub y: Y,
    }

    impl core::ops::AddAssign<W> for XY {
        fn add_assign(&mut self, other: W) {
            self.x += other;
        }
    }

    impl core::ops::Add<W> for XY {
        type Output = Self;

        fn add(mut self, other: W) -> Self::Output {
            self += other;
            self
        }
    }

    impl core::ops::AddAssign<H> for XY {
        fn add_assign(&mut self, other: H) {
            self.y += other;
        }
    }

    impl core::ops::Add<H> for XY {
        type Output = Self;

        fn add(mut self, other: H) -> Self::Output {
            self += other;
            self
        }
    }

    impl core::ops::SubAssign<W> for XY {
        fn sub_assign(&mut self, other: W) {
            self.x -= other;
        }
    }

    impl core::ops::Sub<W> for XY {
        type Output = Self;

        fn sub(mut self, other: W) -> Self::Output {
            self -= other;
            self
        }
    }

    impl core::ops::SubAssign<H> for XY {
        fn sub_assign(&mut self, other: H) {
            self.y -= other;
        }
    }

    impl core::ops::Sub<H> for XY {
        type Output = Self;

        fn sub(mut self, other: H) -> Self::Output {
            self -= other;
            self
        }
    }

    impl core::ops::Sub for XY {
        type Output = WH;

        fn sub(self, other: XY) -> Self::Output {
            WH {
                w: W(self.x.0 - other.x.0),
                h: H(self.y.0 - other.y.0),
            }
        }
    }

    impl core::ops::MulAssign<Inner> for W {
        fn mul_assign(&mut self, inner: Inner) {
            self.0 *= inner;
        }
    }

    impl core::ops::Mul<Inner> for W {
        type Output = Self;

        fn mul(mut self, inner: Inner) -> Self::Output {
            self *= inner;
            self
        }
    }

    impl core::ops::Mul<W> for Inner {
        type Output = W;

        fn mul(self, mut w: W) -> Self::Output {
            w *= self;
            w
        }
    }

    impl core::ops::DivAssign<Inner> for W {
        fn div_assign(&mut self, inner: Inner) {
            self.0 /= inner;
        }
    }

    impl core::ops::Div<Inner> for W {
        type Output = Self;

        fn div(mut self, inner: Inner) -> Self::Output {
            self /= inner;
            self
        }
    }

    impl core::ops::MulAssign<Inner> for H {
        fn mul_assign(&mut self, inner: Inner) {
            self.0 *= inner;
        }
    }

    impl core::ops::Mul<Inner> for H {
        type Output = Self;

        fn mul(mut self, inner: Inner) -> Self::Output {
            self *= inner;
            self
        }
    }

    impl core::ops::Mul<H> for Inner {
        type Output = H;

        fn mul(self, mut h: H) -> Self::Output {
            h *= self;
            h
        }
    }

    impl core::ops::DivAssign<Inner> for H {
        fn div_assign(&mut self, inner: Inner) {
            self.0 /= inner;
        }
    }

    impl core::ops::Div<Inner> for H {
        type Output = Self;

        fn div(mut self, inner: Inner) -> Self::Output {
            self /= inner;
            self
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct WH {
        pub w: W,
        pub h: H,
    }

    impl core::ops::AddAssign for WH {
        fn add_assign(&mut self, other: WH) {
            self.w += other.w;
            self.h += other.h;
        }
    }

    impl core::ops::Add for WH {
        type Output = Self;

        fn add(mut self, other: WH) -> Self::Output {
            self += other;
            self
        }
    }

    impl core::ops::SubAssign for WH {
        fn sub_assign(&mut self, other: WH) {
            self.w -= other.w;
            self.h -= other.h;
        }
    }

    impl core::ops::Sub for WH {
        type Output = Self;

        fn sub(mut self, other: WH) -> Self::Output {
            self -= other;
            self
        }
    }

    impl core::ops::MulAssign<Inner> for WH {
        fn mul_assign(&mut self, inner: Inner) {
            self.w *= inner;
            self.h *= inner;
        }
    }

    impl core::ops::Mul<Inner> for WH {
        type Output = Self;

        fn mul(mut self, inner: Inner) -> Self::Output {
            self *= inner;
            self
        }
    }

    impl core::ops::AddAssign<WH> for XY {
        fn add_assign(&mut self, other: WH) {
            self.x += other.w;
            self.y += other.h;
        }
    }

    impl core::ops::Add<WH> for XY {
        type Output = Self;

        fn add(mut self, other: WH) -> Self::Output {
            self += other;
            self
        }
    }

    impl core::ops::AddAssign<W> for WH {
        fn add_assign(&mut self, other: W) {
            self.w += other;
        }
    }

    impl core::ops::Add<W> for WH {
        type Output = Self;

        fn add(mut self, other: W) -> Self::Output {
            self += other;
            self
        }
    }

    impl core::ops::AddAssign<H> for WH {
        fn add_assign(&mut self, other: H) {
            self.h += other;
        }
    }

    impl core::ops::Add<H> for WH {
        type Output = Self;

        fn add(mut self, other: H) -> Self::Output {
            self += other;
            self
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Rect {
        pub x: X,
        pub y: Y,
        pub w: W,
        pub h: H,
    }

    impl Rect {
        pub fn xy(self) -> XY {
            XY {
                x: self.x,
                y: self.y,
            }
        }

        pub fn wh(self) -> WH {
            WH {
                w: self.w,
                h: self.h,
            }
        }

        pub const fn xy_wh(xy: XY, wh: WH) -> Rect {
            Rect {
                x: xy.x,
                y: xy.y,
                w: wh.w,
                h: wh.h,
            }
        }
    }
}

pub const GFX_WIDTH: usize = 512;
pub const GFX_HEIGHT: usize = 256;
pub const GFX_LENGTH: usize = GFX_WIDTH * GFX_HEIGHT;

pub const FONT_BASE_Y: u8 = 128;
pub const FONT_WIDTH: usize = 128;
pub const FONT_HEIGHT: usize = 128;
pub const FONT_LENGTH: usize = FONT_WIDTH * FONT_HEIGHT;

pub type PaletteIndex = u8;


pub mod sprite {
    pub use super::unscaled::{W, H};

    pub type Inner = u16;
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct X(pub Inner);
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct Y(pub Inner);

    impl From<X> for usize {
        fn from(x: X) -> Self {
            x.0.into()
        }
    }

    impl From<Y> for usize {
        fn from(y: Y) -> Self {
            y.0.into()
        }
    }

    impl core::ops::AddAssign<W> for X {
        fn add_assign(&mut self, other: W) {
            self.0 += other.0;
        }
    }

    impl core::ops::Add<W> for X {
        type Output = Self;

        fn add(mut self, other: W) -> Self::Output {
            self += other;
            self
        }
    }

    pub const fn x_const_add_w(x: X, w: W) -> X {
        X(x.0 + w.0)
    }

    impl core::ops::AddAssign<H> for Y {
        fn add_assign(&mut self, other: H) {
            self.0 += other.0;
        }
    }

    impl core::ops::Add<H> for Y {
        type Output = Self;

        fn add(mut self, other: H) -> Self::Output {
            self += other;
            self
        }
    }

    pub const fn y_const_add_h(y: Y, h: H) -> Y {
        Y(y.0 + h.0)
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct XY {
        pub x: X,
        pub y: Y,
    }

    impl core::ops::AddAssign<W> for XY {
        fn add_assign(&mut self, other: W) {
            self.x += other;
        }
    }

    impl core::ops::Add<W> for XY {
        type Output = Self;

        fn add(mut self, other: W) -> Self::Output {
            self += other;
            self
        }
    }

    impl core::ops::AddAssign<H> for XY {
        fn add_assign(&mut self, other: H) {
            self.y += other;
        }
    }

    impl core::ops::Add<H> for XY {
        type Output = Self;

        fn add(mut self, other: H) -> Self::Output {
            self += other;
            self
        }
    }
}

pub mod command {
    use xs::Xs;
    use super::{ARGB, sprite, unscaled};

    pub type Inner = unscaled::Inner;

    // Small enough to fit on pretty much any reasonable device, at an aspect ratio
    // of 3:2 (1.5), which is a compromise between 4:3 (1.33...) and 16:9 (1.788...).
    //pub const WIDTH: Inner = 480;
    //pub const HEIGHT: Inner = 320;

    // Need more pixels to fit the current art
    pub const WIDTH: Inner = 45 * 16;
    pub const HEIGHT: Inner = 45 * 16;

    pub const LENGTH: usize = WIDTH as usize * HEIGHT as usize;

    pub const WIDTH_W: unscaled::W = unscaled::W(WIDTH);
    pub const HEIGHT_H: unscaled::H = unscaled::H(HEIGHT);

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct X(unscaled::X);

    impl X {
        pub const MAX: X = X(unscaled::X(WIDTH - 1));

        pub const fn get(self) -> unscaled::X {
            self.0
        }

        pub const fn clipped(x: unscaled::X) -> X {
            if x.0 < X::MAX.0.0 {
                X(x)
            } else {
                X::MAX
            }
        }

        pub const fn clipped_inner(x: Inner) -> X {
            X::clipped(unscaled::X(x))
        }

        pub fn gen(rng: &mut Xs) -> X {
            X::clipped(unscaled::X(xs::range(rng, 0..WIDTH as _) as Inner))
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct Y(unscaled::Y);

    impl Y {
        pub const MAX: Y = Y(unscaled::Y(HEIGHT - 1));

        pub const fn get(self) -> unscaled::Y {
            self.0
        }

        pub const fn clipped(y: unscaled::Y) -> Y {
            if y.0 < Y::MAX.0.0 {
                Y(y)
            } else {
                Y::MAX
            }
        }

        pub const fn clipped_inner(y: Inner) -> Y {
            Y::clipped(unscaled::Y(y))
        }

        pub fn gen(rng: &mut Xs) -> Y {
            Y::clipped(unscaled::Y(xs::range(rng, 0..WIDTH as _) as Inner))
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct W(unscaled::W);

    impl W {
        pub const MAX: W = W(unscaled::W(WIDTH - 1));

        pub const fn get(self) -> unscaled::W {
            self.0
        }

        pub const fn clipped(w: unscaled::W) -> W {
            if w.0 < W::MAX.0.0 {
                W(w)
            } else {
                W::MAX
            }
        }

        pub const fn clipped_inner(w: Inner) -> W {
            W::clipped(unscaled::W(w))
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct H(unscaled::H);

    impl H {
        pub const MAX: H = H(unscaled::H(WIDTH - 1));

        pub const fn get(self) -> unscaled::H {
            self.0
        }

        pub const fn clipped(h: unscaled::H) -> H {
            if h.0 < H::MAX.0.0 {
                H(h)
            } else {
                H::MAX
            }
        }

        pub const fn clipped_inner(h: Inner) -> H {
            H::clipped(unscaled::H(h))
        }
    }

    pub const fn w_to_usize(w: W) -> usize {
        w.0.0 as usize
    }

    pub const fn h_to_usize(h: H) -> usize {
        h.0.0 as usize
    }

    pub const fn w_const_add(a: W, b: W) -> W {
        W::clipped_inner(a.0.0 + b.0.0)
    }

    pub const fn w_const_sub(a: W, b: W) -> W {
        W::clipped_inner(a.0.0 - b.0.0)
    }

    pub const fn w_const_mul(a: W, b: Inner) -> W {
        W::clipped_inner(a.0.0 * b)
    }

    pub const fn w_const_div(a: W, b: Inner) -> W {
        W::clipped_inner(a.0.0 / b)
    }

    pub const fn h_const_add(a: H, b: H) -> H {
        H::clipped_inner(a.0.0 + b.0.0)
    }

    pub const fn h_const_sub(a: H, b: H) -> H {
        H::clipped_inner(a.0.0 - b.0.0)
    }

    pub const fn h_const_mul(a: H, b: Inner) -> H {
        H::clipped_inner(a.0.0 * b)
    }

    pub const fn h_const_div(a: H, b: Inner) -> H {
        H::clipped_inner(a.0.0 / b)
    }

    impl From<X> for usize {
        fn from(x: X) -> Self {
            x.0.0.into()
        }
    }

    impl From<Y> for usize {
        fn from(y: Y) -> Self {
            y.0.0.into()
        }
    }

    impl From<X> for Inner {
        fn from(to_convert: X) -> Inner {
            to_convert.0.0
        }
    }

    impl From<Y> for Inner {
        fn from(to_convert: Y) -> Inner {
            to_convert.0.0
        }
    }

    impl From<W> for Inner {
        fn from(to_convert: W) -> Inner {
            to_convert.0.0
        }
    }

    impl From<H> for Inner {
        fn from(to_convert: H) -> Inner {
            to_convert.0.0
        }
    }

    impl core::ops::AddAssign<W> for X {
        fn add_assign(&mut self, other: W) {
            *self = Self::clipped(self.0 + other.0);
        }
    }

    impl core::ops::Add<W> for X {
        type Output = Self;

        fn add(mut self, other: W) -> Self::Output {
            self += other;
            self
        }
    }

    impl core::ops::AddAssign<H> for Y {
        fn add_assign(&mut self, other: H) {
            *self = Self::clipped(self.0 + other.0);
        }
    }

    impl core::ops::Add<H> for Y {
        type Output = Self;

        fn add(mut self, other: H) -> Self::Output {
            self += other;
            self
        }
    }

    impl core::ops::MulAssign<Inner> for W {
        fn mul_assign(&mut self, inner: Inner) {
            *self = Self::clipped(self.0 * inner);
        }
    }

    impl core::ops::Mul<Inner> for W {
        type Output = Self;

        fn mul(mut self, inner: Inner) -> Self::Output {
            self *= inner;
            self
        }
    }

    impl core::ops::Mul<W> for Inner {
        type Output = W;

        fn mul(self, mut w: W) -> Self::Output {
            w *= self;
            w
        }
    }

    impl core::ops::MulAssign<Inner> for H {
        fn mul_assign(&mut self, inner: Inner) {
            *self = Self::clipped(self.0 * inner);
        }
    }

    impl core::ops::Mul<Inner> for H {
        type Output = Self;

        fn mul(mut self, inner: Inner) -> Self::Output {
            self *= inner;
            self
        }
    }

    impl core::ops::Mul<H> for Inner {
        type Output = H;

        fn mul(self, mut h: H) -> Self::Output {
            h *= self;
            h
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct Rect {
        pub x_min: X,
        pub y_min: Y,
        pub x_max: X,
        pub y_max: Y,
    }

    impl Rect {
        pub fn from_unscaled(
            unscaled::Rect {
                x,
                y,
                w,
                h,
            }: unscaled::Rect
        ) -> Rect {
            Rect {
                x_min: X::clipped(x),
                y_min: Y::clipped(y),
                x_max: X::clipped_inner((x + w).get() - 1),
                y_max: Y::clipped_inner((y + h).get() - 1),
            }
        }

        pub fn unscaled(self) -> unscaled::Rect {
            let Rect {
                x_min,
                y_min,
                x_max,
                y_max,
            }: Rect = self;

            unscaled::Rect {
                x: x_min.get(),
                y: y_min.get(),
                w: x_max.get() - x_min.get() + unscaled::W(1),
                h: y_max.get() - y_min.get() + unscaled::H(1),
            }
        }
    }

    #[test]
    fn from_unscaled_then_unscaled_is_identity_on_this_example() {
        let expected = Rect {
            x_min: X::clipped_inner(2),
            y_min: Y::clipped_inner(3),
            x_max: X::clipped_inner(5),
            y_max: Y::clipped_inner(7),
        };

        let actual = Rect::from_unscaled(expected.unscaled());

        assert_eq!(expected, actual);
    }

    #[test]
    fn unscaled_then_from_unscaled_is_identity_on_this_example() {
        let expected = unscaled::Rect {
            x: unscaled::X(7),
            y: unscaled::Y(5),
            w: unscaled::W(3),
            h: unscaled::H(2),
        };

        let actual = Rect::from_unscaled(expected).unscaled();

        assert_eq!(expected, actual);
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Command {
        pub rect: Rect,
        pub sprite_xy: sprite::XY,
        pub colour_override: ARGB,
    }    
}
pub use command::Command;

#[derive(Clone, Copy, Default, Debug)]
pub struct Input {
    pub gamepad: Button,
    pub previous_gamepad: Button,
}

impl Input {
    #[allow(unused)]
    pub fn pressed_this_frame(&self, buttons: Button) -> bool {
        !self.previous_gamepad.contains(buttons) && self.gamepad.contains(buttons)
    }

    #[allow(unused)]
    pub fn released_this_frame(&self, buttons: Button) -> bool {
        self.previous_gamepad.contains(buttons) && !self.gamepad.contains(buttons)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SFX {
    CardPlace,
    CardSlide,
    ButtonPress,
}

pub struct Speaker {
    requests: Vec<SFX>,
}

impl Default for Speaker {
    fn default() -> Self {
        Speaker {
            requests: Vec::with_capacity(8),
        }
    }
}

impl Speaker {
    pub fn clear(&mut self) {
        self.requests.clear();
    }

    pub fn request_sfx(&mut self, sfx: SFX) {
        self.requests.push(sfx);
    }

    pub fn slice(&self) -> &[SFX] {
        &self.requests
    }
}

// These values are deliberately picked to be the same as the ones in NES' input registers.
pub mod button {
    #[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
    pub struct Button(u8);

    impl Button {
        pub const A     : Self = Self(1 << 0);
        pub const B     : Self = Self(1 << 1);
        pub const SELECT: Self = Self(1 << 2);
        pub const START : Self = Self(1 << 3);
        pub const UP    : Self = Self(1 << 4);
        pub const DOWN  : Self = Self(1 << 5);
        pub const LEFT  : Self = Self(1 << 6);
        pub const RIGHT : Self = Self(1 << 7);

        pub const ALL : [Self; 8] = [
            Self::A,
            Self::B,
            Self::SELECT,
            Self::START,
            Self::UP,
            Self::DOWN,
            Self::LEFT,
            Self::RIGHT,
        ];

        pub const fn contains(&self, other: Self) -> bool {
            self.0 & other.0 == other.0
        }

        pub fn insert(&mut self, other: Self) {
            self.0 |= other.0;
        }

        pub fn remove(&mut self, other: Self) {
            self.0 &= !other.0;
        }
    }
}
pub use button::Button;

pub type Logger = Option<fn(&str) -> ()>;

pub type StateParams = ([u8; 16], Logger, Logger);

pub trait State {
    fn frame(&mut self) -> (&[Command], &[SFX]);

    fn press(&mut self, button: Button);

    fn release(&mut self, button: Button);
}

// reportedly colourblind friendly colours
// https://twitter.com/ea_accessible/status/968595073184092160
pub mod colours {
    use super::ARGB;

    pub const BLUE: ARGB = 0xFF3352E1;
    pub const GREEN: ARGB = 0xFF30B06E;
    pub const RED: ARGB = 0xFFDE4949;
    pub const YELLOW: ARGB = 0xFFFFB937;
    pub const PURPLE: ARGB = 0xFF533354;
    #[allow(unused)]
    pub const GREY: ARGB = 0xFF5A7D8B;
    #[allow(unused)]
    pub const GRAY: ARGB = GREY;
    pub const WHITE: ARGB = 0xFFEEEEEE;
    pub const BLACK: ARGB = 0xFF222222;
}

pub use colours::*;

pub const PALETTE: [ARGB; 8] = [
    BLUE,
    GREEN,
    RED,
    YELLOW,
    PURPLE,
    GREY,
    WHITE,
    BLACK,
];