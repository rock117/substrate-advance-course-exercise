#[cfg(feature = "runtime-benchmarks")]

  use crate::{*, Module as PalletModule};
  use frame_benchmarking::{benchmarks, account, impl_benchmark_test_suite};
  use frame_system::RawOrigin;

  benchmarks!{
    // Individual benchmarks are placed here
    do_something {
        let b in 1 .. 1000
        let caller = account("caller", 0, 0);
      }:  do_something(caller, b.into())
      verify {
        let value = Something::<T>::get();
		assert_eq!(value, b.into());
      }
  }



#[cfg(test)]
mod tests {
	use super::*;
	use crate::mock::{new_test_ext, Test};
	use frame_support::assert_ok;

	#[test]
	fn test_benchmarks() {
		new_test_ext().execute_with(|| {
			assert_ok!(test_benchmark_do_something::<Test>());
		});
	}
}
