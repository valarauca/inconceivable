/*
 * Copyright (c) 2020 William Cody Laeder
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */
#![cfg_attr(not(feature = "std"), no_std)]

/// inconceivable is a macro which closely parallels `std::unreachable`, or `std::panic`.
///
/// The primary difference is that when this crate is
/// configured with the `ub_inconceivable` option it will emit
/// the `core::hint::unreachable_unchecked` to hint
/// for the compiler to understand a condition should
/// never occur.
///
/// Generally compilers assume UB won't happen. This macro
/// offers the "best of both worlds", it provides a solid
/// way of asserting/testing behavior in debug builds, and
/// no cost in properly configured release builds.
#[macro_export]
macro_rules! inconceivable {
    () => {
        {
        #[cfg(all(feature = "ub_inconceivable", feature = "RUSTC_VERSION_GE_1_27", not(feature = "std")))]
        {
            unsafe{ core::hint::unreachable_unchecked() }
        }

        #[cfg(all(feature = "ub_inconceivable", feature = "RUSTC_VERSION_GE_1_27", feature = "std"))]
        {
            unsafe{ std::hint::unreachable_unchecked() }
        }


        #[cfg(not(all(feature = "ub_inconceivable", feature = "RUSTC_VERSION_GE_1_27")))]
        {
            unreachable!()
        }
        }
    };
    ($msg: expr) => {
        {
        #[cfg(all(feature = "ub_inconceivable", feature = "RUSTC_VERSION_GE_1_27", not(feature = "std")))]
        {
            unsafe{ core::hint::unreachable_unchecked() }
        }

        #[cfg(all(feature = "ub_inconceivable", feature = "RUSTC_VERSION_GE_1_27", feature = "std"))]
        {
            unsafe{ std::hint::unreachable_unchecked() }
        }


        #[cfg(not(all(feature = "ub_inconceivable", feature = "RUSTC_VERSION_GE_1_27")))]
        {
            unreachable!($msg)
        }
        }
    };
    ($msg: expr,) => {
        {
        #[cfg(all(feature = "ub_inconceivable", feature = "RUSTC_VERSION_GE_1_27", not(feature = "std")))]
        {
            unsafe{ core::hint::unreachable_unchecked() }
        }

        #[cfg(all(feature = "ub_inconceivable", feature = "RUSTC_VERSION_GE_1_27", feature = "std"))]
        {
            unsafe{ std::hint::unreachable_unchecked() }
        }


        #[cfg(not(all(feature = "ub_inconceivable", feature = "RUSTC_VERSION_GE_1_27")))]
        {
            unreachable!($msg)
        }
        }
    };
    ($fmt: expr, $($arg:tt)*) => {
        {
        #[cfg(all(feature = "ub_inconceivable", feature = "RUSTC_VERSION_GE_1_27", not(feature = "std")))]
        {
            unsafe{ core::hint::unreachable_unchecked() }
        }

        #[cfg(all(feature = "ub_inconceivable", feature = "RUSTC_VERSION_GE_1_27", feature = "std"))]
        {
            unsafe{ std::hint::unreachable_unchecked() }
        }

        #[cfg(not(all(feature = "ub_inconceivable", feature = "RUSTC_VERSION_GE_1_27")))]
        {
            unreachable!($fmt, $($arg)*)
        }
        }
    };
}

#[cfg(test)]
mod test {

    #[cfg(not(feature = "ub_inconceivable"))]
    #[test]
    #[should_panic]
    fn test_expansion() {
        match 10usize {
            1 => {}
            _ => inconceivable!(),
        };
    }
}
