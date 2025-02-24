#[burn_tensor_testgen::testgen(q_matmul)]
mod tests {
    use super::*;
    use burn_tensor::TensorData;

    // NOTE: we set higher tolerance (0.3) due to larger de/quantization errors accumulation
    #[test]
    fn test_matmul_d2() {
        let tensor_1 = QTensor::<TestBackend, 2>::int8([[1.0, 7.0], [2.0, 3.0], [1.0, 5.0]]);
        let tensor_2 = QTensor::<TestBackend, 2>::int8([[4.0, 7.0, 5.0], [2.0, 3.0, 5.0]]);

        let tensor_3 = tensor_1.matmul(tensor_2);
        let expected =
            TensorData::from([[18.0, 28.0, 40.0], [14.0, 23.0, 25.0], [14.0, 22.0, 30.0]]);

        tensor_3
            .dequantize()
            .into_data()
            .assert_approx_eq_diff(&expected, 0.3);
    }

    #[test]
    fn test_matmul_d3() {
        let tensor_1 = QTensor::<TestBackend, 3>::int8([[[1.0, 7.0], [2.0, 3.0]]]);
        let tensor_2 = QTensor::<TestBackend, 3>::int8([[[4.0, 7.0], [2.0, 3.0]]]);

        let tensor_3 = tensor_1.matmul(tensor_2);
        let expected = TensorData::from([[[18.0, 28.0], [14.0, 23.0]]]);

        tensor_3
            .dequantize()
            .into_data()
            .assert_approx_eq_diff(&expected, 0.3);
    }

    #[test]
    fn test_matmul_broadcast_1() {
        let tensor_1 = QTensor::<TestBackend, 3>::int8([[[1.0, 7.0], [2.0, 3.0]]]);
        let tensor_2 =
            QTensor::<TestBackend, 3>::int8([[[4.0, 7.0], [2.0, 3.0]], [[2.0, 5.0], [6.0, 3.0]]]);

        let tensor_3 = tensor_1.matmul(tensor_2);
        let expected =
            TensorData::from([[[18.0, 28.0], [14.0, 23.0]], [[44.0, 26.0], [22.0, 19.0]]]);

        tensor_3
            .dequantize()
            .into_data()
            .assert_approx_eq_diff(&expected, 0.3);
    }

    #[test]
    fn test_matmul_broadcast_4d() {
        let tensor_1 = QTensor::<TestBackend, 4>::int8([
            [[[1.0, 7.0], [2.0, 3.0]]],
            [[[2.0, 5.0], [6.0, 3.0]]],
        ]);
        let tensor_2 =
            QTensor::<TestBackend, 4>::int8([[[[9.0, 8.0], [1.0, 4.0]], [[2.0, 7.0], [3.0, 5.0]]]]);

        // [2, 1, 2, 2] @ [1, 2, 2, 2] -> [2, 2, 2, 2]
        let tensor_3 = tensor_1.matmul(tensor_2);
        let expected = TensorData::from([
            [[[16.0, 36.0], [21.0, 28.0]], [[23.0, 42.0], [13.0, 29.0]]],
            [[[23.0, 36.0], [57.0, 60.0]], [[19.0, 39.0], [21.0, 57.0]]],
        ]);

        tensor_3
            .dequantize()
            .into_data()
            .assert_approx_eq_diff(&expected, 0.3);
    }

    #[test]
    fn test_matmul_simple_1() {
        // NOTE: we use affine quantization to lower de/quantization errors
        let tensor_1 = QTensor::<TestBackend, 2>::int8_affine([[5.0, 14.0], [14.0, 25.0]]);
        let tensor_2 = QTensor::<TestBackend, 2>::int8_affine([[3.0, 4.0, 5.0], [0.0, 1.0, 2.0]]);

        let tensor_3 = tensor_1.matmul(tensor_2);
        let expected = TensorData::from([[15.0, 34.0, 53.0], [42.0, 81.0, 120.0]]);

        tensor_3
            .dequantize()
            .into_data()
            .assert_approx_eq_diff(&expected, 0.3);
    }

    #[test]
    fn test_matmul_4_3() {
        // NOTE: we use affine quantization to lower de/quantization errors
        let tensor_1 = QTensor::<TestBackend, 2>::int8_affine([
            [0., 1., 2., 3.],
            [4., 5., 6., 7.],
            [8., 9., 10., 11.],
        ]);
        let tensor_2 = QTensor::<TestBackend, 2>::int8_affine([
            [0., 1., 2.],
            [4., 5., 6.],
            [8., 9., 10.],
            [13., 14., 15.],
        ]);

        let tensor_3 = tensor_1.matmul(tensor_2);
        let expected = TensorData::from([[59., 65., 71.], [159., 181., 203.], [259., 297., 335.]]);

        tensor_3
            .dequantize()
            .into_data()
            .assert_approx_eq_diff(&expected, 0.3);
    }

    #[test]
    fn test_matmul_trivial() {
        // NOTE: we use affine quantization to lower de/quantization errors
        let tensor_1 = QTensor::<TestBackend, 2>::int8_affine([
            [0., 1., 2., 3.],
            [4., 5., 6., 7.],
            [8., 9., 10., 11.],
            [12., 13., 14., 15.],
        ]);

        let tensor_3 = tensor_1.clone().matmul(tensor_1);

        tensor_3.dequantize().into_data().assert_approx_eq(
            &TensorData::from([
                [56., 62., 68., 74.],
                [152., 174., 196., 218.],
                [248., 286., 324., 362.],
                [344., 398., 452., 506.],
            ]),
            3,
        );
    }

    #[test]
    fn test_matmul_trivial_transposed() {
        // NOTE: we use affine quantization to lower de/quantization errors
        let tensor_1 = QTensor::<TestBackend, 2>::int8_affine([
            [0., 1., 2., 3.],
            [4., 5., 6., 7.],
            [8., 9., 10., 11.],
            [12., 13., 14., 15.],
        ]);

        let tensor_3 = tensor_1.clone().matmul(tensor_1.transpose());

        tensor_3.dequantize().into_data().assert_approx_eq(
            &TensorData::from([
                [14., 38., 62., 86.],
                [38., 126., 214., 302.],
                [62., 214., 366., 518.],
                [86., 302., 518., 734.],
            ]),
            1,
        );
    }

    #[test]
    fn test_matmul_simple_2() {
        let tensor_1 = QTensor::<TestBackend, 2>::int8([[1.0, 2.0, 3.0, 4.0]]);
        let tensor_2 = QTensor::<TestBackend, 2>::int8([[3.0], [4.0], [5.0], [6.0]]);

        let tensor_3 = tensor_1.matmul(tensor_2);
        let expected = TensorData::from([[50.0]]);

        tensor_3
            .dequantize()
            .into_data()
            .assert_approx_eq_diff(&expected, 0.3);
    }

    #[test]
    fn test_matmul_simple_3() {
        let tensor_1 = QTensor::<TestBackend, 2>::int8([
            [3., 3., 3.],
            [4., 4., 4.],
            [5., 5., 5.],
            [6., 6., 6.],
        ]);
        let tensor_2 =
            QTensor::<TestBackend, 2>::int8([[1., 2., 3., 4.], [1., 2., 3., 4.], [1., 2., 3., 4.]]);

        let tensor_3 = tensor_1.matmul(tensor_2);
        let expected = TensorData::from([
            [9., 18., 27., 36.],
            [12., 24., 36., 48.],
            [15., 30., 45., 60.],
            [18., 36., 54., 72.],
        ]);

        tensor_3
            .dequantize()
            .into_data()
            .assert_approx_eq_diff(&expected, 0.3);
    }

    #[test]
    #[should_panic]
    fn should_panic_when_inner_dimensions_are_not_equal() {
        let tensor_1 = QTensor::<TestBackend, 2>::int8([[3., 3.], [4., 4.], [5., 5.], [6., 6.]]);
        let tensor_2 =
            QTensor::<TestBackend, 2>::int8([[1., 2., 3., 4.], [1., 2., 3., 4.], [1., 2., 3., 4.]]);

        let _ = tensor_1.matmul(tensor_2);
    }
}
