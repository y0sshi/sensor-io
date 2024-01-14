use super::num_traits;
use super::image::GenericImageView;
use super::ndarray;

pub struct NDRaw<T: num_traits::PrimInt> {
    data: ndarray::Array2<T>,
}
impl<T: num_traits::PrimInt> NDRaw<T> {
    // 画サイズ指定コンストラクタ
    pub fn new(width: usize, height: usize) -> Self {
        let data    = ndarray::Array2::<T>::zeros((height, width));
        NDRaw {data}
    }

    // Vector2D変換コンストラクタ
    pub fn new_from_vector2d(vec2d: &Vec<Vec<T>>) -> Self {
        let vec1d = Self::convert_vector2d_to_vector1d(&vec2d);
        let data  = Self::convert_vector1d_to_ndarray(vec1d, vec2d[0].len(), vec2d.len());
        NDRaw {data}
    }

    // image(RGB)変換コンストラクタ
    pub fn new_from_rgbimage(path_image_in: String) -> Self {
        let img_in = image::open(path_image_in).unwrap();
        let data   = Self::convert_rgb_to_ndarray(&img_in);
        NDRaw {data}
    }

    // data取得
    pub fn data(&self) -> &ndarray::Array2<T> {
        &self.data
    }

    // pix取得
    pub fn pix(&self, x: usize, y: usize) -> T {
        self.data.row(y)[x]
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

    fn convert_vector1d_to_ndarray(vec1d: Vec<T>, width: usize, height: usize) -> ndarray::Array2<T> {
        ndarray::Array2::from_shape_vec((height, width), vec1d).unwrap()
    }

    fn convert_vector2d_to_vector1d(vec2d: &Vec<Vec<T>>) -> Vec<T> {
        let mut vec1d = Vec::<T>::new();
        for row in vec2d {
            vec1d.extend(row);
        }
        vec1d
    }

    fn convert_rgb_to_ndarray(img_in: &image::DynamicImage) -> ndarray::Array2<T> {
        ndarray::Array2::<T>::from_shape_fn(
            (img_in.height() as usize, img_in.width()  as usize),
            |(y, x)| -> T {
                Self::convert_rgb_to_bayer(img_in, x, y)
            })
    }

    fn convert_rgb_to_bayer(img_in: &image::DynamicImage, x: usize, y: usize) -> T {
        let pix;
        if y % 2 == 0 {
            if x % 2 == 0 {
                // R
                pix = T::from(img_in.get_pixel(x as u32, y as u32)[0]);
            } else {
                // Gr
                pix = T::from(img_in.get_pixel(x as u32, y as u32)[1]);
            }
        } else {
            if x % 2 == 0 {
                // Gb
                pix = T::from(img_in.get_pixel(x as u32, y as u32)[1]);
            } else {
                // B
                pix = T::from(img_in.get_pixel(x as u32, y as u32)[2]);
            }
        }
        pix.expect("convert Rgba -> T")
    }
}

#[cfg(test)]
mod test {
    use super::NDRaw;

    #[test]
    fn test_new() {
        let raw_in = NDRaw::<u16>::new(3, 2);
        println!("raw_in.width()  = {}", raw_in.width());
        println!("raw_in.height() = {}", raw_in.height());
        assert_eq!(3, raw_in.width());
        assert_eq!(2, raw_in.height());
    }

    #[test]
    fn test_new_from_vector() {
        let vec2d: Vec<Vec<u16>> = vec![
            vec![0, 1, 2, 3],
            vec![4, 5, 6, 7],
            vec![8, 9,10,11],
        ];
        let raw_in = NDRaw::<u16>::new_from_vector2d(&vec2d);
        println!("raw_in.width()          = {}", raw_in.width());
        println!("raw_in.height()         = {}", raw_in.height());
        println!("raw_in.data()           = \n{}", raw_in.data());
        println!("raw_in.data().row(1)    = {}", raw_in.data().row(1));
        println!("raw_in.data().column(1) = {}", raw_in.data().column(1));
        for y in 0 .. vec2d.len() {
            for x in 0 .. vec2d[0].len() {
                println!("vec2d[y][x]:{} == raw_in.pix(x, y):{}", vec2d[y][x], raw_in.pix(x, y));
                assert_eq!(vec2d[y][x], raw_in.pix(x, y));
            }
        }
    }
}

