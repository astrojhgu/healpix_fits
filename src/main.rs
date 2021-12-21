extern crate healpix_fits;
use scorus::{
    healpix::{
        utils::nside2npix
    }
};

use healpix_fits::{
    write_map
    , read_map
};

fn main(){
    let nside=128;
    //let data=vec![0.0; nside2npix(nside)];
    let data:Vec<_>=(0..nside2npix(nside)).map(|i|(i%2) as f64).collect();
    write_map("a.fits", &[&data], false, true);
    let data1=read_map::<f64>("a.fits", &["TEMPERATURE"], 1);
    data.iter().zip(data1[0].iter()).for_each(|(&a,&b)|{assert_eq!(a,b)});
}
