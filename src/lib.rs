pub mod sensor_io {
    use image::GenericImageView;
    use num_traits;
    use nalgebra;

    pub struct NARaw<T: num_traits::PrimInt + nalgebra::Scalar> {
        data: nalgebra::DMatrix<T>
    }
    impl<T: num_traits::PrimInt + nalgebra::Scalar> NARaw<T> {
        // 画サイズ指定コンストラクタ
        pub fn new(width: usize, height: usize) -> Self {
            let data = nalgebra::DMatrix::<T>::zeros(height, width);
            NARaw {data}
        }

        // Vector2D変換コンストラクタ
        pub fn new_from_vector2d(vec2d: &Vec<Vec<T>>) -> Self {
            let data = Self::convert_vector2d_to_dmatrix(&vec2d);
            NARaw {data}
        }

        // image(RGB)変換コンストラクタ
        pub fn new_from_rgbimage(path_image_in: String) -> Self {
            let img_in = image::open(path_image_in).unwrap();
            let data   = Self::convert_rgb_to_dmatrix(&img_in);
            NARaw {data}
        }

        // data取得
        pub fn data(&self) -> &nalgebra::DMatrix<T> {
            &self.data
        }

        // pix取得
        pub fn pix(&self, x: usize, y: usize) -> T {
            self.data.column(x)[y]
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

        fn convert_vector2d_to_dmatrix(vec2d: &Vec<Vec<T>>) -> nalgebra::DMatrix<T> {
            nalgebra::DMatrix::<T>::from_fn(
                vec2d.len(),
                vec2d[0].len(),
                |y, x| -> T {
                    vec2d[y][x]
                })
        }

        fn convert_rgb_to_dmatrix(img_in: &image::DynamicImage) -> nalgebra::DMatrix<T> {
            nalgebra::DMatrix::<T>::from_fn(
                img_in.height() as usize,
                img_in.width()  as usize,
                |y, x| -> T {
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
}

#[cfg(test)]
mod test {
    use super::sensor_io;

    #[test]
    fn test_new() {
        let raw_in = sensor_io::NARaw::<u16>::new(3, 2);
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
        let raw_in = sensor_io::NARaw::<u16>::new_from_vector2d(&vec2d);
        println!("raw_in.width()          = {}", raw_in.width());
        println!("raw_in.height()         = {}", raw_in.height());
        println!("raw_in.data()           = {}", raw_in.data());
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

