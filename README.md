# A thin wrapper over [fitsio](https://crates.io/crates/fitsio) for simple healpix file I/O

Usage: check [src/main.rs](src/main.rs)

## Example
```RUST
extern crate healpix_fits;

use healpix_fits::{
    write_map
    , read_map
    , nside2npix
};

fn main(){
    let nside=128;
    // fill the output data
    let data:Vec<_>=(0..nside2npix(nside)).map(|i|(i%2) as f64).collect();

    //write to a fits file
    write_map("a.fits", &[&data], false, true);

    //read it back, here we read only the "TEMPERATURE" column (which is the only column)
    let data1=read_map::<f64>("a.fits", &["TEMPERATURE"], 1);

    //check whether they are identical to the previous written data
    data.iter().zip(data1[0].iter()).for_each(|(&a,&b)|{assert_eq!(a,b)});
}
```
