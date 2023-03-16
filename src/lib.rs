use fitsio::{
    FitsFile
    , tables::{
        ColumnDescription
        , ColumnDataType
        , WritesCol
        , ReadsCol
    }, hdu::FitsHdu
};

pub fn isqrt(x: usize) -> usize
{
    (x as f64 + 0.5).sqrt() as usize
}

pub fn nside2npix(nside: usize)->usize{
    nside.pow(2)*12
}

pub fn npix2nside(npix: usize) -> usize {
    let res = isqrt(npix / 12);
    if nside2npix(res) == npix {
        res
    } else {
        panic!()
    }
}




pub fn standard_column_names(n: usize)->Vec<String>{
    match n{
        1=>vec!["TEMPERATURE"].iter().map(|&s| s.to_string()).collect(),
        2=>vec!["Q_POLARISATION", "U_POLARISATION"].iter().map(|&s| s.to_string()).collect(),
        3=>vec!["TEMPERATURE", "Q_POLARISATION", "U_POLARISATION"].iter().map(|&s| s.to_string()).collect(),
        6=>vec!["II", "IQ", "IU", "QQ", "QU", "UU"].iter().map(|&s| s.to_string()).collect(),
        _=>panic!("number of columns not equal to either 1, 2, 3, or 6")
    }
}

pub trait AllowedColumnDataType:WritesCol{
    fn get_column_data_type()->ColumnDataType;
}

impl AllowedColumnDataType for f32{
    fn get_column_data_type() ->ColumnDataType {
        ColumnDataType::Float
    }
}

impl AllowedColumnDataType for f64{
    fn get_column_data_type() ->ColumnDataType {
        ColumnDataType::Double
    }
}


pub fn write_map<T>
(
    filename: &str,
    map_data: &[&[T]],
    nest: bool,
    overwrite: bool
)->FitsHdu
where T:AllowedColumnDataType
{
    let nside=npix2nside(map_data[0].len());
    let mut fitsfile=if overwrite {
        FitsFile::create(filename).overwrite().open().unwrap()
    }else{
        FitsFile::create(filename).open().unwrap()
    };
    let column_names=standard_column_names(map_data.len());
    let column_desc:Vec<_>=column_names.iter().map(|name| ColumnDescription::new(name)
    .with_type(T::get_column_data_type())
    .with_width(1)
    .that_repeats(1)
    .create().unwrap()).collect();
    let hdu=fitsfile.create_table("Healpix map", &column_desc).unwrap();
    column_names.iter().zip(map_data.iter()).for_each(|(n,&d)|{
        hdu.write_col(&mut fitsfile, n, d).unwrap();
    });
    hdu.write_key(&mut fitsfile, "PIXTYPE", "HEALPIX").unwrap();
    hdu.write_key(&mut fitsfile, "ORDERING", if nest {"NESTED"} else {"RING"}).unwrap();
    hdu.write_key(&mut fitsfile, "NSIDE", nside as i32).unwrap();
    hdu.write_key(&mut fitsfile, "INDXSCHM", "IMPLICIT").unwrap();
    hdu.write_key(&mut fitsfile, "OBJECT", "FULLSKY").unwrap();
    hdu 
}

pub fn read_map<T>(
    filename: &str
    , col_names: &[&str]
    , hdu: usize
)->Vec<Vec<T>>
where T: ReadsCol{
    let mut fitsfile=FitsFile::open(filename).unwrap();
    let hdu=fitsfile.hdu(hdu).unwrap();
    col_names.iter().map(|&name|{
        hdu.read_col::<T>(&mut fitsfile, name).unwrap()
    }).collect()
}
