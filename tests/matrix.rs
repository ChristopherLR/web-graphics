use web_graphics::math::matrix::*;

#[test]
fn matrix_invert() {
  let mat = Matrix::new();
  let inverse = mat.invert().unwrap();

  let ans: [f32; 16] = [
    1., 0., 0., 0.,
    0., 1., 0., 0.,
    0., 0., 1., 0.,
    0., 0., 0., 1.,
  ];

  assert_eq!(ans, inverse.0)
}

#[test]
fn matrix_invert2() {
  let mut mat = Matrix::new();
  let rhs: [f32; 16] = [
    1.0, 2.0, 3.0, 74.0,
    5.0, 13.0, 7.0, 8.0,
    9.0, 10.0, 11.0, 12.0,
    13.0, 14.0, 15.0, 16.0,
  ];
  mat.set_arr(rhs);

  let mut inverse = mat.invert().unwrap();

  let mut ans: [f32; 16] = [
    1.0/140.0, -1.0/14.0, -491.0/280.0, 369.0/280.0,
    0.0, 1.0/7.0, -2.0/7.0, 1.0/7.0,
    -3.0/140.0, -1.0/14.0, 513.0/280.0, -347.0/280.0,
    1.0/70.0, 0.0, -3.0/70.0, 1.0/35.0,
  ];

  let precision: usize = 6;
  let a1 = inverse.print_std(precision);
  let mut ans_mat = Matrix::new();
  ans_mat.set_arr(ans);
  let a2 = ans_mat.print_std(precision);

  assert_eq!(a1, a2)
}