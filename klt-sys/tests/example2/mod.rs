use klt;
use std::{ptr, slice};
use std::os::raw;


pub unsafe fn unsafe_main() {
    let n_features = 100;

    let tc = klt::KLTCreateTrackingContext();
    klt::KLTPrintTrackingContext(tc);
    let fl = klt::KLTCreateFeatureList(n_features);
    let flr = &mut *fl;

    let mut ncols = 0;
    let mut nrows = 0;
    let img1 = klt::pgmReadFile(s!("img0.pgm"), ptr::null(), &mut ncols, &mut nrows);
    let img2 = klt::pgmReadFile(s!("img1.pgm"), ptr::null(), &mut ncols, &mut nrows);

    klt::KLTSelectGoodFeatures(tc, img1, ncols, nrows, fl);

    klt::KLTWriteFeatureListToPPM(fl, img1, ncols, nrows, s!("feat1.ppm"));
    klt::KLTWriteFeatureList(fl, s!("feat1.txt"), s!("%3d"));

    klt::KLTTrackFeatures(tc, img1, img2, ncols, nrows, fl);
    klt::KLTReplaceLostFeatures(tc, img2, ncols, nrows, fl);

    klt::KLTWriteFeatureListToPPM(fl, img2, ncols, nrows, s!("feat2.ppm"));
    klt::KLTWriteFeatureList(fl, s!("feat2.txt"), s!("%3d"));
}

