use byteorder::{ReadBytesExt, WriteBytesExt};
use image::GenericImageView;
use nalgebra;
use num_traits;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub struct NARaw<T: num_traits::PrimInt + num_traits::FromPrimitive + nalgebra::Scalar> {
    data: nalgebra::DMatrix<T>,
}
impl<T: num_traits::PrimInt + num_traits::FromPrimitive + nalgebra::Scalar> NARaw<T> {
    // 画サイズ指定コンストラクタ
    pub fn new(width: usize, height: usize) -> Self {
        let data = nalgebra::DMatrix::<T>::zeros(height, width);
        NARaw { data }
    }

    // Vector2D変換コンストラクタ
    pub fn new_from_vector2d(vec2d: &[Vec<T>]) -> Self {
        let data = Self::convert_vector2d_to_dmatrix(vec2d);
        NARaw { data }
    }

    // image(bin)変換コンストラクタ
    pub fn new_from_binimage(path_raw_in: String) -> Self {
        let mut f_read = BufReader::new(File::open(path_raw_in).unwrap());

        let width = f_read.read_u16::<byteorder::LittleEndian>().unwrap() as usize; // Little Endian(u16)
        let height = f_read.read_u16::<byteorder::LittleEndian>().unwrap() as usize; // Little Endian(u16)
        let mut data = nalgebra::DMatrix::<T>::zeros(height, width);
        for y in 0..height {
            for x in 0..width {
                data[(y, x)] =
                    T::from_u16(f_read.read_u16::<byteorder::LittleEndian>().unwrap()).unwrap();
            }
        }

        NARaw { data }
    }

    // image(RGB)変換コンストラクタ
    pub fn new_from_rgbimage(path_image_in: String) -> Self {
        let img_in = image::open(path_image_in).unwrap();
        let data = Self::convert_rgb_to_dmatrix(&img_in);
        NARaw { data }
    }

    // data取得
    pub fn data(&self) -> &nalgebra::DMatrix<T> {
        &self.data
    }

    // pix取得
    pub fn pix(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.data[(y, x)]
    }

    // 形状取得
    pub fn shape(&self) -> (usize, usize) {
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
                    .write_u16::<byteorder::LittleEndian>(self.data[(y, x)].to_u16().unwrap());
            }
        }

        self
    }

    // bin画像読み込み
    pub fn read_binimage(&mut self, path_raw_in: String) -> &Self {
        *self = Self::new_from_binimage(path_raw_in);

        self
    }
    fn convert_vector2d_to_dmatrix(vec2d: &[Vec<T>]) -> nalgebra::DMatrix<T> {
        nalgebra::DMatrix::<T>::from_fn(vec2d.len(), vec2d[0].len(), |y, x| -> T { vec2d[y][x] })
    }

    fn convert_rgb_to_dmatrix(img_in: &image::DynamicImage) -> nalgebra::DMatrix<T> {
        nalgebra::DMatrix::<T>::from_fn(
            img_in.height() as usize,
            img_in.width() as usize,
            |y, x| -> T { Self::convert_rgb_to_bayer(img_in, x, y) },
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
    use super::NARaw;

    #[test]
    fn test_new() {
        println!("naraw::test::test_new()  {{");

        let raw_in = NARaw::<u16>::new(3, 2);
        println!("  [naraw][test_new()] raw_in.width()  = {}", raw_in.width());
        println!(
            "  [naraw][test_new()] raw_in.height() = {}",
            raw_in.height()
        );
        assert_eq!(3, raw_in.width());
        assert_eq!(2, raw_in.height());

        println!("}}");
    }

    #[test]
    fn test_new_from_vector() {
        println!("naraw::test::test_new_from_vector()  {{");

        let vec2d: Vec<Vec<u16>> = vec![vec![0, 1, 2, 3], vec![4, 5, 6, 7], vec![8, 9, 10, 11]];
        let mut raw_in = NARaw::<u16>::new_from_vector2d(&vec2d);
        println!(
            "  [naraw][test_new_from_vector()] raw_in.width()          = {}",
            raw_in.width()
        );
        println!(
            "  [naraw][test_new_from_vector()] raw_in.height()         = {}",
            raw_in.height()
        );
        println!(
            "  [naraw][test_new_from_vector()] raw_in.data()           = {}",
            raw_in.data()
        );
        println!(
            "  [naraw][test_new_from_vector()] raw_in.data().row(1)    = {}",
            raw_in.data().row(1)
        );
        println!(
            "  [naraw][test_new_from_vector()] raw_in.data().column(1) = {}",
            raw_in.data().column(1)
        );
        for y in 0..vec2d.len() {
            for x in 0..vec2d[0].len() {
                println!(
                    "  [naraw][test_new_from_vector()] vec2d[y][x]:{} == raw_in.pix(x, y):{}",
                    vec2d[y][x],
                    raw_in.pix(x, y)
                );
                assert_eq!(vec2d[y][x], *raw_in.pix(x, y));
            }
        }

        *raw_in.pix(2, 1) = 30;
        println!(
            "  [naraw][test_new_from_vector()] raw_in.data()           = \n{}",
            raw_in.data()
        );

        raw_in.write_binimage(String::from("write_naraw.bin"));

        println!("}}");
    }
}
