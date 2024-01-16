use byteorder::{ReadBytesExt, WriteBytesExt};
use image::GenericImageView;
use ndarray;
use num_traits;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub struct NDRaw<T: num_traits::PrimInt + num_traits::FromPrimitive + num_traits::ToPrimitive> {
    data: ndarray::Array2<T>,
}
impl<T: num_traits::PrimInt + num_traits::FromPrimitive + num_traits::ToPrimitive> NDRaw<T> {
    // 画サイズ指定コンストラクタ
    pub fn new(width: usize, height: usize) -> Self {
        let data = ndarray::Array2::<T>::zeros((height, width));
        NDRaw { data }
    }

    // Vector2D変換コンストラクタ
    pub fn new_from_vector2d(vec2d: &[Vec<T>]) -> Self {
        let vec1d = Self::convert_vector2d_to_vector1d(vec2d);
        let data = Self::convert_vector1d_to_ndarray(vec1d, vec2d[0].len(), vec2d.len());
        NDRaw { data }
    }

    // image(bin)変換コンストラクタ
    pub fn new_from_binimage(path_raw_in: String) -> Self {
        let mut f_read = BufReader::new(File::open(path_raw_in).unwrap());

        let width = f_read.read_u16::<byteorder::LittleEndian>().unwrap() as usize; // Little Endian(u16)
        let height = f_read.read_u16::<byteorder::LittleEndian>().unwrap() as usize; // Little Endian(u16)
        let mut data = ndarray::Array2::<T>::zeros((height, width));
        for y in 0..height {
            for x in 0..width {
                data[[y, x]] =
                    T::from_u16(f_read.read_u16::<byteorder::LittleEndian>().unwrap()).unwrap();
            }
        }

        NDRaw { data }
    }

    // image(RGB)変換コンストラクタ
    pub fn new_from_rgbimage(path_image_in: String) -> Self {
        let img_in = image::open(path_image_in).unwrap();
        let data = Self::convert_rgb_to_ndarray(&img_in);
        NDRaw { data }
    }

    // data取得
    pub fn data(&self) -> &ndarray::Array2<T> {
        &self.data
    }

    // pix取得
    pub fn pix(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[[y, x]]
    }

    // 形状取得
    pub fn shape(&self) -> &[usize] {
        self.data.shape()
    }

    // width取得
    pub fn width(&self) -> usize {
        self.data.ncols()
    }

    // height取得
    pub fn height(&self) -> usize {
        self.data.nrows()
    }

    // bin画像書き込み
    pub fn write_binimage(&self, path_raw_out: String) -> &Self {
        let mut f_write = BufWriter::new(File::create(path_raw_out).unwrap());

        let width = Self::width(self);
        let height = Self::height(self);
        let _ = f_write.write_u16::<byteorder::LittleEndian>(width as u16);
        let _ = f_write.write_u16::<byteorder::LittleEndian>(height as u16);
        for y in 0..height {
            for x in 0..width {
                let _ = f_write
                    .write_u16::<byteorder::LittleEndian>(self.data[[y, x]].to_u16().unwrap());
            }
        }

        self
    }

    // bin画像読み込み
    pub fn read_binimage(&mut self, path_raw_in: String) -> &Self {
        *self = Self::new_from_binimage(path_raw_in);

        self
    }

    fn convert_vector1d_to_ndarray(
        vec1d: Vec<T>,
        width: usize,
        height: usize,
    ) -> ndarray::Array2<T> {
        ndarray::Array2::from_shape_vec((height, width), vec1d).unwrap()
    }

    fn convert_vector2d_to_vector1d(vec2d: &[Vec<T>]) -> Vec<T> {
        let mut vec1d = Vec::<T>::new();
        for row in vec2d {
            vec1d.extend(row);
        }
        vec1d
    }

    fn convert_rgb_to_ndarray(img_in: &image::DynamicImage) -> ndarray::Array2<T> {
        ndarray::Array2::<T>::from_shape_fn(
            (img_in.height() as usize, img_in.width() as usize),
            |(y, x)| -> T { Self::convert_rgb_to_bayer(img_in, x, y) },
        )
    }

    fn convert_rgb_to_bayer(img_in: &image::DynamicImage, x: usize, y: usize) -> T {
        let pix;
        if x % 2 != y % 2 {
            // G
            pix = T::from(img_in.get_pixel(x as u32, y as u32)[1]).unwrap();
        } else if x % 2 == 0 {
            // R
            pix = T::from(img_in.get_pixel(x as u32, y as u32)[0]).unwrap();
        } else {
            // B
            pix = T::from(img_in.get_pixel(x as u32, y as u32)[2]).unwrap();
        }
        pix
    }
}

#[cfg(test)]
mod test {
    use super::NDRaw;

    #[test]
    fn test_new() {
        println!("ndraw::test::test_new()  {{");

        let raw_in = NDRaw::<u16>::new(3, 2);
        println!("  [ndraw][test_new()] raw_in.width()  = {}", raw_in.width());
        println!(
            "  [ndraw][test_new()] raw_in.height() = {}",
            raw_in.height()
        );
        assert_eq!(3, raw_in.width());
        assert_eq!(2, raw_in.height());

        println!("}}");
    }

    #[test]
    fn test_new_from_vector() {
        println!("ndraw::test::test_new_from_vector()  {{");

        let vec2d: Vec<Vec<u16>> = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
        let mut raw_in = NDRaw::<u16>::new_from_vector2d(vec2d);
        println!(
            "  [ndraw][test_new_from_vector()] raw_in.width()          = {}",
            raw_in.width()
        );
        println!(
            "  [ndraw][test_new_from_vector()] raw_in.height()         = {}",
            raw_in.height()
        );
        println!(
            "  [ndraw][test_new_from_vector()] raw_in.data()           = \n{}",
            raw_in.data()
        );
        println!(
            "  [ndraw][test_new_from_vector()] raw_in.data().row(1)    = {}",
            raw_in.data().row(1)
        );
        println!(
            "  [ndraw][test_new_from_vector()] raw_in.data().column(1) = {}",
            raw_in.data().column(1)
        );
        for y in 0..vec2d.len() {
            for x in 0..vec2d[0].len() {
                println!(
                    "  [ndraw][test_new_from_vector()] vec2d[y][x]:{} == raw_in.pix(x, y):{}",
                    vec2d[y][x],
                    raw_in.pix(x, y)
                );
                assert_eq!(vec2d[y][x], *raw_in.pix(x, y));
            }
        }
        *raw_in.pix(2, 1) = 20;
        println!(
            "  [ndraw][test_new_from_vector()] raw_in.data()           = \n{}",
            raw_in.data()
        );

        raw_in.write_binimage(String::from("write_ndraw.bin"));
        println!("}}");
    }

    #[test]
    fn test_new_from_binimage() {
        println!("ndraw::test::test_new_from_vector()  {{");

        let raw_in = NDRaw::<u16>::new_from_binimage(String::from("a.bin"));
        println!(
            "  [ndraw][test_new_from_binimage()] raw_in.width()  = {}",
            raw_in.width()
        );
        println!(
            "  [ndraw][test_new_from_binimage()] raw_in.height() = {}",
            raw_in.height()
        );
        println!(
            "  [ndraw][test_new_from_binimage()] raw_in.data()           = \n{}",
            raw_in.data()
        );
        println!(
            "  [ndraw][test_new_from_binimage()] raw_in.data().row(1)    = {}",
            raw_in.data().row(1)
        );
        println!(
            "  [ndraw][test_new_from_binimage()] raw_in.data().column(1) = {}",
            raw_in.data().column(1)
        );
        assert_eq!(4, raw_in.width());
        assert_eq!(3, raw_in.height());

        println!("}}");
    }
}
