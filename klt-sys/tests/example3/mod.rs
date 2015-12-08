use klt;
use std::ptr;
use std::os::raw;
use std::ffi::CString;

const REPLACE: bool = false;

pub unsafe fn unsafe_main() {
    let n_features = 150;
    let n_frames = 10;

    let tc = klt::KLTCreateTrackingContext();
    let fl = klt::KLTCreateFeatureList(n_features);
    let ft = klt::KLTCreateFeatureTable(n_frames, n_features);
    let tcr = &mut *tc;

    tcr.sequential_mode = true as klt::Bool;
    tcr.write_internal_images = false as klt::Bool;
    tcr.affine_consistency_check = -1;

    let mut ncols = 0;
    let mut nrows = 0;
    let img1 = klt::pgmReadFile(s!("img0.pgm"), ptr::null_mut(), &mut ncols, &mut nrows);
    let mut img2 = Vec::<raw::c_uchar>::with_capacity((nrows * ncols) as usize);

    klt::KLTSelectGoodFeatures(tc, img1, ncols, nrows, fl);
    klt::KLTStoreFeatureList(fl, ft, 0);
    klt::KLTWriteFeatureListToPPM(fl, img1, ncols, nrows, s!("feat0.ppm"));

    let mut fnamein;
    let mut fnameout;
    for i in 1..n_frames {
        fnamein = CString::new(format!("img{}.pgm", i)).unwrap();
        klt::pgmReadFile(fnamein.as_ptr(), img2.as_mut_ptr(), &mut ncols, &mut nrows);
        klt::KLTTrackFeatures(tc, img1, img2.as_ptr(), ncols, nrows, fl);
        if REPLACE {
            klt::KLTReplaceLostFeatures(tc, img2.as_ptr(), ncols, nrows, fl);
        }
        klt::KLTStoreFeatureList(fl, ft, i);
        fnameout = CString::new(format!("feat{}.ppm", i)).unwrap();
        klt::KLTWriteFeatureListToPPM(fl, img2.as_ptr(), ncols, nrows, fnameout.as_ptr());
    }
    klt::KLTWriteFeatureTable(ft, s!("features.txt"), s!("%5.1f"));
    klt::KLTWriteFeatureTable(ft, s!("features.ft"), ptr::null());

    klt::KLTFreeFeatureTable(ft);
    klt::KLTFreeFeatureList(fl);
    klt::KLTFreeTrackingContext(tc);
}

