use klt;
use std::ptr;
use std::os::raw;


pub unsafe fn unsafe_main() {
    let n_features = 100;

    let tc = klt::KLTCreateTrackingContext();
    let tcr = &mut *tc;
    tcr.mindist = 20;
    tcr.window_width  = 9;
    tcr.window_height = 9;
    klt::KLTChangeTCPyramid(tc, 15);
    klt::KLTUpdateTCBorder(tc);
    let fl = klt::KLTCreateFeatureList(n_features);

    let mut ncols = 0;
    let mut nrows = 0;
    let img1 = klt::pgmReadFile(s!("img0.pgm"), ptr::null_mut(), &mut ncols, &mut nrows);
    let img2 = klt::pgmReadFile(s!("img2.pgm"), ptr::null_mut(), &mut ncols, &mut nrows);

    klt::KLTSelectGoodFeatures(tc, img1, ncols, nrows, fl);

    klt::KLTWriteFeatureListToPPM(fl, img1, ncols, nrows, s!("feat1b.ppm"));

    klt::KLTTrackFeatures(tc, img1, img2, ncols, nrows, fl);

    klt::KLTWriteFeatureListToPPM(fl, img2, ncols, nrows, s!("feat2b.ppm"));
}

