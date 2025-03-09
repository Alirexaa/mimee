/// This module defines the `MimeDict` struct which provides a mapping between file extensions and their corresponding MIME types.
use std::collections::HashMap;

/// The `MimeDict` struct contains a single field:
/// - `mime_types`: A `HashMap<String, String>` that stores the mapping between file extensions and MIME types.
pub struct MimeDict {
    mime_types: HashMap<String, String>,
}

impl MimeDict {
    /// Creates a new `MimeDict` instance and initializes it with a predefined set of file extensions and their corresponding MIME types.
    pub fn new() -> MimeDict {
        let mut mime_types: HashMap<String, String> = HashMap::new();
        mime_types.insert(".323".to_string(), "text/h323".to_string());
        mime_types.insert(".3g2".to_string(), "video/3gpp2".to_string());
        mime_types.insert(".3gp2".to_string(), "video/3gpp2".to_string());
        mime_types.insert(".3gp".to_string(), "video/3gpp".to_string());
        mime_types.insert(".3gpp".to_string(), "video/3gpp".to_string());
        mime_types.insert(".aac".to_string(), "audio/aac".to_string());
        mime_types.insert(".aaf".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".aca".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".accdb".to_string(), "application/msaccess".to_string());
        mime_types.insert(".accde".to_string(), "application/msaccess".to_string());
        mime_types.insert(".accdt".to_string(), "application/msaccess".to_string());
        mime_types.insert(
            ".acx".to_string(),
            "application/internet-property-stream".to_string(),
        );
        mime_types.insert(".adt".to_string(), "audio/vnd.dlna.adts".to_string());
        mime_types.insert(".adts".to_string(), "audio/vnd.dlna.adts".to_string());
        mime_types.insert(".afm".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".ai".to_string(), "application/postscript".to_string());
        mime_types.insert(".aif".to_string(), "audio/x-aiff".to_string());
        mime_types.insert(".aifc".to_string(), "audio/aiff".to_string());
        mime_types.insert(".aiff".to_string(), "audio/aiff".to_string());
        mime_types.insert(".appcache".to_string(), "text/cache-manifest".to_string());
        mime_types.insert(
            ".application".to_string(),
            "application/x-ms-application".to_string(),
        );
        mime_types.insert(".art".to_string(), "image/x-jg".to_string());
        mime_types.insert(".asd".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".asf".to_string(), "video/x-ms-asf".to_string());
        mime_types.insert(".asi".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".asm".to_string(), "text/plain".to_string());
        mime_types.insert(".asr".to_string(), "video/x-ms-asf".to_string());
        mime_types.insert(".asx".to_string(), "video/x-ms-asf".to_string());
        mime_types.insert(".atom".to_string(), "application/atom+xml".to_string());
        mime_types.insert(".au".to_string(), "audio/basic".to_string());
        mime_types.insert(".avi".to_string(), "video/x-msvideo".to_string());
        mime_types.insert(".axs".to_string(), "application/olescript".to_string());
        mime_types.insert(".bas".to_string(), "text/plain".to_string());
        mime_types.insert(".bcpio".to_string(), "application/x-bcpio".to_string());
        mime_types.insert(".bin".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".bmp".to_string(), "image/bmp".to_string());
        mime_types.insert(".c".to_string(), "text/plain".to_string());
        mime_types.insert(
            ".cab".to_string(),
            "application/vnd.ms-cab-compressed".to_string(),
        );
        mime_types.insert(
            ".calx".to_string(),
            "application/vnd.ms-office.calx".to_string(),
        );
        mime_types.insert(
            ".cat".to_string(),
            "application/vnd.ms-pki.seccat".to_string(),
        );
        mime_types.insert(".cdf".to_string(), "application/x-cdf".to_string());
        mime_types.insert(".chm".to_string(), "application/octet-stream".to_string());
        mime_types.insert(
            ".class".to_string(),
            "application/x-java-applet".to_string(),
        );
        mime_types.insert(".clp".to_string(), "application/x-msclip".to_string());
        mime_types.insert(".cmx".to_string(), "image/x-cmx".to_string());
        mime_types.insert(".cnf".to_string(), "text/plain".to_string());
        mime_types.insert(".cod".to_string(), "image/cis-cod".to_string());
        mime_types.insert(".cpio".to_string(), "application/x-cpio".to_string());
        mime_types.insert(".cpp".to_string(), "text/plain".to_string());
        mime_types.insert(".crd".to_string(), "application/x-mscardfile".to_string());
        mime_types.insert(".crl".to_string(), "application/pkix-crl".to_string());
        mime_types.insert(".crt".to_string(), "application/x-x509-ca-cert".to_string());
        mime_types.insert(".csh".to_string(), "application/x-csh".to_string());
        mime_types.insert(".css".to_string(), "text/css".to_string());
        mime_types.insert(".csv".to_string(), "text/csv".to_string());
        mime_types.insert(".cur".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".dcr".to_string(), "application/x-director".to_string());
        mime_types.insert(
            ".deploy".to_string(),
            "application/octet-stream".to_string(),
        );
        mime_types.insert(".der".to_string(), "application/x-x509-ca-cert".to_string());
        mime_types.insert(".dib".to_string(), "image/bmp".to_string());
        mime_types.insert(".dir".to_string(), "application/x-director".to_string());
        mime_types.insert(".disco".to_string(), "text/xml".to_string());
        mime_types.insert(".dlm".to_string(), "text/dlm".to_string());
        mime_types.insert(".doc".to_string(), "application/msword".to_string());
        mime_types.insert(
            ".docm".to_string(),
            "application/vnd.ms-word.document.macroEnabled.12".to_string(),
        );
        mime_types.insert(
            ".docx".to_string(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
        );
        mime_types.insert(".dot".to_string(), "application/msword".to_string());
        mime_types.insert(
            ".dotm".to_string(),
            "application/vnd.ms-word.template.macroEnabled.12".to_string(),
        );
        mime_types.insert(
            ".dotx".to_string(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.template".to_string(),
        );
        mime_types.insert(".dsp".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".dtd".to_string(), "text/xml".to_string());
        mime_types.insert(".dvi".to_string(), "application/x-dvi".to_string());
        mime_types.insert(".dvr-ms".to_string(), "video/x-ms-dvr".to_string());
        mime_types.insert(".dwf".to_string(), "drawing/x-dwf".to_string());
        mime_types.insert(".dwp".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".dxr".to_string(), "application/x-director".to_string());
        mime_types.insert(".eml".to_string(), "message/rfc822".to_string());
        mime_types.insert(".emz".to_string(), "application/octet-stream".to_string());
        mime_types.insert(
            ".eot".to_string(),
            "application/vnd.ms-fontobject".to_string(),
        );
        mime_types.insert(".eps".to_string(), "application/postscript".to_string());
        mime_types.insert(".etx".to_string(), "text/x-setext".to_string());
        mime_types.insert(".evy".to_string(), "application/envoy".to_string());
        mime_types.insert(
            ".exe".to_string(),
            "application/vnd.microsoft.portable-executable".to_string(),
        );
        mime_types.insert(".fdf".to_string(), "application/vnd.fdf".to_string());
        mime_types.insert(".fif".to_string(), "application/fractals".to_string());
        mime_types.insert(".fla".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".flr".to_string(), "x-world/x-vrml".to_string());
        mime_types.insert(".flv".to_string(), "video/x-flv".to_string());
        mime_types.insert(".gif".to_string(), "image/gif".to_string());
        mime_types.insert(".gtar".to_string(), "application/x-gtar".to_string());
        mime_types.insert(".gz".to_string(), "application/x-gzip".to_string());
        mime_types.insert(".h".to_string(), "text/plain".to_string());
        mime_types.insert(".hdf".to_string(), "application/x-hdf".to_string());
        mime_types.insert(".hdml".to_string(), "text/x-hdml".to_string());
        mime_types.insert(".hhc".to_string(), "application/x-oleobject".to_string());
        mime_types.insert(".hhk".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".hhp".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".hlp".to_string(), "application/winhlp".to_string());
        mime_types.insert(".hqx".to_string(), "application/mac-binhex40".to_string());
        mime_types.insert(".hta".to_string(), "application/hta".to_string());
        mime_types.insert(".htc".to_string(), "text/x-component".to_string());
        mime_types.insert(".htm".to_string(), "text/html".to_string());
        mime_types.insert(".html".to_string(), "text/html".to_string());
        mime_types.insert(".htt".to_string(), "text/webviewhtml".to_string());
        mime_types.insert(".hxt".to_string(), "text/html".to_string());
        mime_types.insert(".ical".to_string(), "text/calendar".to_string());
        mime_types.insert(".icalendar".to_string(), "text/calendar".to_string());
        mime_types.insert(".ico".to_string(), "image/x-icon".to_string());
        mime_types.insert(".ics".to_string(), "text/calendar".to_string());
        mime_types.insert(".ief".to_string(), "image/ief".to_string());
        mime_types.insert(".ifb".to_string(), "text/calendar".to_string());
        mime_types.insert(".iii".to_string(), "application/x-iphone".to_string());
        mime_types.insert(".inf".to_string(), "application/octet-stream".to_string());
        mime_types.insert(
            ".ins".to_string(),
            "application/x-internet-signup".to_string(),
        );
        mime_types.insert(
            ".isp".to_string(),
            "application/x-internet-signup".to_string(),
        );
        mime_types.insert(".IVF".to_string(), "video/x-ivf".to_string());
        mime_types.insert(".jar".to_string(), "application/java-archive".to_string());
        mime_types.insert(".java".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".jck".to_string(), "application/liquidmotion".to_string());
        mime_types.insert(".jcz".to_string(), "application/liquidmotion".to_string());
        mime_types.insert(".jfif".to_string(), "image/pjpeg".to_string());
        mime_types.insert(".jpb".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".jpe".to_string(), "image/jpeg".to_string());
        mime_types.insert(".jpeg".to_string(), "image/jpeg".to_string());
        mime_types.insert(".jpg".to_string(), "image/jpeg".to_string());
        mime_types.insert(".js".to_string(), "text/javascript".to_string());
        mime_types.insert(".json".to_string(), "application/json".to_string());
        mime_types.insert(".jsx".to_string(), "text/jscript".to_string());
        mime_types.insert(".latex".to_string(), "application/x-latex".to_string());
        mime_types.insert(".lit".to_string(), "application/x-ms-reader".to_string());
        mime_types.insert(".lpk".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".lsf".to_string(), "video/x-la-asf".to_string());
        mime_types.insert(".lsx".to_string(), "video/x-la-asf".to_string());
        mime_types.insert(".lzh".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".m13".to_string(), "application/x-msmediaview".to_string());
        mime_types.insert(".m14".to_string(), "application/x-msmediaview".to_string());
        mime_types.insert(".m1v".to_string(), "video/mpeg".to_string());
        mime_types.insert(".m2ts".to_string(), "video/vnd.dlna.mpeg-tts".to_string());
        mime_types.insert(".m3u".to_string(), "audio/x-mpegurl".to_string());
        mime_types.insert(".m4a".to_string(), "audio/mp4".to_string());
        mime_types.insert(".m4v".to_string(), "video/mp4".to_string());
        mime_types.insert(".man".to_string(), "application/x-troff-man".to_string());
        mime_types.insert(
            ".manifest".to_string(),
            "application/x-ms-manifest".to_string(),
        );
        mime_types.insert(".map".to_string(), "text/plain".to_string());
        mime_types.insert(".markdown".to_string(), "text/markdown".to_string());
        mime_types.insert(".md".to_string(), "text/markdown".to_string());
        mime_types.insert(".mdb".to_string(), "application/x-msaccess".to_string());
        mime_types.insert(".mdp".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".me".to_string(), "application/x-troff-me".to_string());
        mime_types.insert(".mht".to_string(), "message/rfc822".to_string());
        mime_types.insert(".mhtml".to_string(), "message/rfc822".to_string());
        mime_types.insert(".mid".to_string(), "audio/mid".to_string());
        mime_types.insert(".midi".to_string(), "audio/mid".to_string());
        mime_types.insert(".mix".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".mjs".to_string(), "text/javascript".to_string());
        mime_types.insert(".mmf".to_string(), "application/x-smaf".to_string());
        mime_types.insert(".mno".to_string(), "text/xml".to_string());
        mime_types.insert(".mny".to_string(), "application/x-msmoney".to_string());
        mime_types.insert(".mov".to_string(), "video/quicktime".to_string());
        mime_types.insert(".movie".to_string(), "video/x-sgi-movie".to_string());
        mime_types.insert(".mp2".to_string(), "video/mpeg".to_string());
        mime_types.insert(".mp3".to_string(), "audio/mpeg".to_string());
        mime_types.insert(".mp4".to_string(), "video/mp4".to_string());
        mime_types.insert(".mp4v".to_string(), "video/mp4".to_string());
        mime_types.insert(".mpa".to_string(), "video/mpeg".to_string());
        mime_types.insert(".mpe".to_string(), "video/mpeg".to_string());
        mime_types.insert(".mpeg".to_string(), "video/mpeg".to_string());
        mime_types.insert(".mpg".to_string(), "video/mpeg".to_string());
        mime_types.insert(".mpp".to_string(), "application/vnd.ms-project".to_string());
        mime_types.insert(".mpv2".to_string(), "video/mpeg".to_string());
        mime_types.insert(".ms".to_string(), "application/x-troff-ms".to_string());
        mime_types.insert(".msi".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".mso".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".mvb".to_string(), "application/x-msmediaview".to_string());
        mime_types.insert(
            ".mvc".to_string(),
            "application/x-miva-compiled".to_string(),
        );
        mime_types.insert(".nc".to_string(), "application/x-netcdf".to_string());
        mime_types.insert(".nsc".to_string(), "video/x-ms-asf".to_string());
        mime_types.insert(".nws".to_string(), "message/rfc822".to_string());
        mime_types.insert(".ocx".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".oda".to_string(), "application/oda".to_string());
        mime_types.insert(".odc".to_string(), "text/x-ms-odc".to_string());
        mime_types.insert(".ods".to_string(), "application/oleobject".to_string());
        mime_types.insert(".oga".to_string(), "audio/ogg".to_string());
        mime_types.insert(".ogg".to_string(), "video/ogg".to_string());
        mime_types.insert(".ogv".to_string(), "video/ogg".to_string());
        mime_types.insert(".ogx".to_string(), "application/ogg".to_string());
        mime_types.insert(".one".to_string(), "application/onenote".to_string());
        mime_types.insert(".onea".to_string(), "application/onenote".to_string());
        mime_types.insert(".onetoc".to_string(), "application/onenote".to_string());
        mime_types.insert(".onetoc2".to_string(), "application/onenote".to_string());
        mime_types.insert(".onetmp".to_string(), "application/onenote".to_string());
        mime_types.insert(".onepkg".to_string(), "application/onenote".to_string());
        mime_types.insert(
            ".osdx".to_string(),
            "application/opensearchdescription+xml".to_string(),
        );
        mime_types.insert(".otf".to_string(), "font/otf".to_string());
        mime_types.insert(".p10".to_string(), "application/pkcs10".to_string());
        mime_types.insert(".p12".to_string(), "application/x-pkcs12".to_string());
        mime_types.insert(
            ".p7b".to_string(),
            "application/x-pkcs7-certificates".to_string(),
        );
        mime_types.insert(".p7c".to_string(), "application/pkcs7-mime".to_string());
        mime_types.insert(".p7m".to_string(), "application/pkcs7-mime".to_string());
        mime_types.insert(
            ".p7r".to_string(),
            "application/x-pkcs7-certreqresp".to_string(),
        );
        mime_types.insert(
            ".p7s".to_string(),
            "application/pkcs7-signature".to_string(),
        );
        mime_types.insert(".pbm".to_string(), "image/x-portable-bitmap".to_string());
        mime_types.insert(".pcx".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".pcz".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".pdf".to_string(), "application/pdf".to_string());
        mime_types.insert(".pfb".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".pfm".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".pfx".to_string(), "application/x-pkcs12".to_string());
        mime_types.insert(".pgm".to_string(), "image/x-portable-graymap".to_string());
        mime_types.insert(".pko".to_string(), "application/vnd.ms-pki.pko".to_string());
        mime_types.insert(".pma".to_string(), "application/x-perfmon".to_string());
        mime_types.insert(".pmc".to_string(), "application/x-perfmon".to_string());
        mime_types.insert(".pml".to_string(), "application/x-perfmon".to_string());
        mime_types.insert(".pmr".to_string(), "application/x-perfmon".to_string());
        mime_types.insert(".pmw".to_string(), "application/x-perfmon".to_string());
        mime_types.insert(".png".to_string(), "image/png".to_string());
        mime_types.insert(".pnm".to_string(), "image/x-portable-anymap".to_string());
        mime_types.insert(".pnz".to_string(), "image/png".to_string());
        mime_types.insert(
            ".pot".to_string(),
            "application/vnd.ms-powerpoint".to_string(),
        );
        mime_types.insert(
            ".potm".to_string(),
            "application/vnd.ms-powerpoint.template.macroEnabled.12".to_string(),
        );
        mime_types.insert(
            ".potx".to_string(),
            "application/vnd.openxmlformats-officedocument.presentationml.template".to_string(),
        );
        mime_types.insert(
            ".ppam".to_string(),
            "application/vnd.ms-powerpoint.addin.macroEnabled.12".to_string(),
        );
        mime_types.insert(".ppm".to_string(), "image/x-portable-pixmap".to_string());
        mime_types.insert(
            ".pps".to_string(),
            "application/vnd.ms-powerpoint".to_string(),
        );
        mime_types.insert(
            ".ppsm".to_string(),
            "application/vnd.ms-powerpoint.slideshow.macroEnabled.12".to_string(),
        );
        mime_types.insert(
            ".ppsx".to_string(),
            "application/vnd.openxmlformats-officedocument.presentationml.slideshow".to_string(),
        );
        mime_types.insert(
            ".ppt".to_string(),
            "application/vnd.ms-powerpoint".to_string(),
        );
        mime_types.insert(
            ".pptm".to_string(),
            "application/vnd.ms-powerpoint.presentation.macroEnabled.12".to_string(),
        );
        mime_types.insert(
            ".pptx".to_string(),
            "application/vnd.openxmlformats-officedocument.presentationml.presentation".to_string(),
        );
        mime_types.insert(".prf".to_string(), "application/pics-rules".to_string());
        mime_types.insert(".prm".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".prx".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".ps".to_string(), "application/postscript".to_string());
        mime_types.insert(".psd".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".psm".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".psp".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".pub".to_string(), "application/x-mspublisher".to_string());
        mime_types.insert(".qt".to_string(), "video/quicktime".to_string());
        mime_types.insert(
            ".qtl".to_string(),
            "application/x-quicktimeplayer".to_string(),
        );
        mime_types.insert(".qxd".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".ra".to_string(), "audio/x-pn-realaudio".to_string());
        mime_types.insert(".ram".to_string(), "audio/x-pn-realaudio".to_string());
        mime_types.insert(".rar".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".ras".to_string(), "image/x-cmu-raster".to_string());
        mime_types.insert(".rf".to_string(), "image/vnd.rn-realflash".to_string());
        mime_types.insert(".rgb".to_string(), "image/x-rgb".to_string());
        mime_types.insert(
            ".rm".to_string(),
            "application/vnd.rn-realmedia".to_string(),
        );
        mime_types.insert(".rmi".to_string(), "audio/mid".to_string());
        mime_types.insert(".roff".to_string(), "application/x-troff".to_string());
        mime_types.insert(
            ".rpm".to_string(),
            "audio/x-pn-realaudio-plugin".to_string(),
        );
        mime_types.insert(".rtf".to_string(), "application/rtf".to_string());
        mime_types.insert(".rtx".to_string(), "text/richtext".to_string());
        mime_types.insert(".scd".to_string(), "application/x-msschedule".to_string());
        mime_types.insert(".sct".to_string(), "text/scriptlet".to_string());
        mime_types.insert(".sea".to_string(), "application/octet-stream".to_string());
        mime_types.insert(
            ".setpay".to_string(),
            "application/set-payment-initiation".to_string(),
        );
        mime_types.insert(
            ".setreg".to_string(),
            "application/set-registration-initiation".to_string(),
        );
        mime_types.insert(".sgml".to_string(), "text/sgml".to_string());
        mime_types.insert(".sh".to_string(), "application/x-sh".to_string());
        mime_types.insert(".shar".to_string(), "application/x-shar".to_string());
        mime_types.insert(".sit".to_string(), "application/x-stuffit".to_string());
        mime_types.insert(
            ".sldm".to_string(),
            "application/vnd.ms-powerpoint.slide.macroEnabled.12".to_string(),
        );
        mime_types.insert(
            ".sldx".to_string(),
            "application/vnd.openxmlformats-officedocument.presentationml.slide".to_string(),
        );
        mime_types.insert(".smd".to_string(), "audio/x-smd".to_string());
        mime_types.insert(".smi".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".smx".to_string(), "audio/x-smd".to_string());
        mime_types.insert(".smz".to_string(), "audio/x-smd".to_string());
        mime_types.insert(".snd".to_string(), "audio/basic".to_string());
        mime_types.insert(".snp".to_string(), "application/octet-stream".to_string());
        mime_types.insert(
            ".spc".to_string(),
            "application/x-pkcs7-certificates".to_string(),
        );
        mime_types.insert(".spl".to_string(), "application/futuresplash".to_string());
        mime_types.insert(".spx".to_string(), "audio/ogg".to_string());
        mime_types.insert(".src".to_string(), "application/x-wais-source".to_string());
        mime_types.insert(".ssm".to_string(), "application/streamingmedia".to_string());
        mime_types.insert(
            ".sst".to_string(),
            "application/vnd.ms-pki.certstore".to_string(),
        );
        mime_types.insert(".stl".to_string(), "application/vnd.ms-pki.stl".to_string());
        mime_types.insert(".sv4cpio".to_string(), "application/x-sv4cpio".to_string());
        mime_types.insert(".sv4crc".to_string(), "application/x-sv4crc".to_string());
        mime_types.insert(".svg".to_string(), "image/svg+xml".to_string());
        mime_types.insert(".svgz".to_string(), "image/svg+xml".to_string());
        mime_types.insert(
            ".swf".to_string(),
            "application/x-shockwave-flash".to_string(),
        );
        mime_types.insert(".t".to_string(), "application/x-troff".to_string());
        mime_types.insert(".tar".to_string(), "application/x-tar".to_string());
        mime_types.insert(".tcl".to_string(), "application/x-tcl".to_string());
        mime_types.insert(".tex".to_string(), "application/x-tex".to_string());
        mime_types.insert(".texi".to_string(), "application/x-texinfo".to_string());
        mime_types.insert(".texinfo".to_string(), "application/x-texinfo".to_string());
        mime_types.insert(".tgz".to_string(), "application/x-compressed".to_string());
        mime_types.insert(
            ".thmx".to_string(),
            "application/vnd.ms-officetheme".to_string(),
        );
        mime_types.insert(".thn".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".tif".to_string(), "image/tiff".to_string());
        mime_types.insert(".tiff".to_string(), "image/tiff".to_string());
        mime_types.insert(".toc".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".tr".to_string(), "application/x-troff".to_string());
        mime_types.insert(".trm".to_string(), "application/x-msterminal".to_string());
        mime_types.insert(".ts".to_string(), "video/vnd.dlna.mpeg-tts".to_string());
        mime_types.insert(".tsv".to_string(), "text/tab-separated-values".to_string());
        mime_types.insert(".ttc".to_string(), "application/x-font-ttf".to_string());
        mime_types.insert(".ttf".to_string(), "application/x-font-ttf".to_string());
        mime_types.insert(".tts".to_string(), "video/vnd.dlna.mpeg-tts".to_string());
        mime_types.insert(".txt".to_string(), "text/plain".to_string());
        mime_types.insert(".u32".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".uls".to_string(), "text/iuls".to_string());
        mime_types.insert(".ustar".to_string(), "application/x-ustar".to_string());
        mime_types.insert(".vbs".to_string(), "text/vbscript".to_string());
        mime_types.insert(".vcf".to_string(), "text/x-vcard".to_string());
        mime_types.insert(".vcs".to_string(), "text/plain".to_string());
        mime_types.insert(
            ".vdx".to_string(),
            "application/vnd.ms-visio.viewer".to_string(),
        );
        mime_types.insert(".vml".to_string(), "text/xml".to_string());
        mime_types.insert(".vsd".to_string(), "application/vnd.visio".to_string());
        mime_types.insert(".vss".to_string(), "application/vnd.visio".to_string());
        mime_types.insert(".vst".to_string(), "application/vnd.visio".to_string());
        mime_types.insert(".vsto".to_string(), "application/x-ms-vsto".to_string());
        mime_types.insert(".vsw".to_string(), "application/vnd.visio".to_string());
        mime_types.insert(".vsx".to_string(), "application/vnd.visio".to_string());
        mime_types.insert(".vtx".to_string(), "application/vnd.visio".to_string());
        mime_types.insert(".wasm".to_string(), "application/wasm".to_string());
        mime_types.insert(".wav".to_string(), "audio/wav".to_string());
        mime_types.insert(".wax".to_string(), "audio/x-ms-wax".to_string());
        mime_types.insert(".wbmp".to_string(), "image/vnd.wap.wbmp".to_string());
        mime_types.insert(".wcm".to_string(), "application/vnd.ms-works".to_string());
        mime_types.insert(".wdb".to_string(), "application/vnd.ms-works".to_string());
        mime_types.insert(".webm".to_string(), "video/webm".to_string());
        mime_types.insert(
            ".webmanifest".to_string(),
            "application/manifest+json".to_string(),
        );
        mime_types.insert(".webp".to_string(), "image/webp".to_string());
        mime_types.insert(".wks".to_string(), "application/vnd.ms-works".to_string());
        mime_types.insert(".wm".to_string(), "video/x-ms-wm".to_string());
        mime_types.insert(".wma".to_string(), "audio/x-ms-wma".to_string());
        mime_types.insert(".wmd".to_string(), "application/x-ms-wmd".to_string());
        mime_types.insert(".wmf".to_string(), "application/x-msmetafile".to_string());
        mime_types.insert(".wml".to_string(), "text/vnd.wap.wml".to_string());
        mime_types.insert(".wmlc".to_string(), "application/vnd.wap.wmlc".to_string());
        mime_types.insert(".wmls".to_string(), "text/vnd.wap.wmlscript".to_string());
        mime_types.insert(
            ".wmlsc".to_string(),
            "application/vnd.wap.wmlscriptc".to_string(),
        );
        mime_types.insert(".wmp".to_string(), "video/x-ms-wmp".to_string());
        mime_types.insert(".wmv".to_string(), "video/x-ms-wmv".to_string());
        mime_types.insert(".wmx".to_string(), "video/x-ms-wmx".to_string());
        mime_types.insert(".wmz".to_string(), "application/x-ms-wmz".to_string());
        mime_types.insert(".woff".to_string(), "application/font-woff".to_string());
        mime_types.insert(".woff2".to_string(), "font/woff2".to_string());
        mime_types.insert(".wps".to_string(), "application/vnd.ms-works".to_string());
        mime_types.insert(".wri".to_string(), "application/x-mswrite".to_string());
        mime_types.insert(".wrl".to_string(), "x-world/x-vrml".to_string());
        mime_types.insert(".wrz".to_string(), "x-world/x-vrml".to_string());
        mime_types.insert(".wsdl".to_string(), "text/xml".to_string());
        mime_types.insert(".wtv".to_string(), "video/x-ms-wtv".to_string());
        mime_types.insert(".wvx".to_string(), "video/x-ms-wvx".to_string());
        mime_types.insert(".x".to_string(), "application/directx".to_string());
        mime_types.insert(".xaf".to_string(), "x-world/x-vrml".to_string());
        mime_types.insert(".xaml".to_string(), "application/xaml+xml".to_string());
        mime_types.insert(
            ".xap".to_string(),
            "application/x-silverlight-app".to_string(),
        );
        mime_types.insert(".xbap".to_string(), "application/x-ms-xbap".to_string());
        mime_types.insert(".xbm".to_string(), "image/x-xbitmap".to_string());
        mime_types.insert(".xdr".to_string(), "text/plain".to_string());
        mime_types.insert(".xht".to_string(), "application/xhtml+xml".to_string());
        mime_types.insert(".xhtml".to_string(), "application/xhtml+xml".to_string());
        mime_types.insert(".xla".to_string(), "application/vnd.ms-excel".to_string());
        mime_types.insert(
            ".xlam".to_string(),
            "application/vnd.ms-excel.addin.macroEnabled.12".to_string(),
        );
        mime_types.insert(".xlc".to_string(), "application/vnd.ms-excel".to_string());
        mime_types.insert(".xlm".to_string(), "application/vnd.ms-excel".to_string());
        mime_types.insert(".xls".to_string(), "application/vnd.ms-excel".to_string());
        mime_types.insert(
            ".xlsb".to_string(),
            "application/vnd.ms-excel.sheet.binary.macroEnabled.12".to_string(),
        );
        mime_types.insert(
            ".xlsm".to_string(),
            "application/vnd.ms-excel.sheet.macroEnabled.12".to_string(),
        );
        mime_types.insert(
            ".xlsx".to_string(),
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string(),
        );
        mime_types.insert(".xlt".to_string(), "application/vnd.ms-excel".to_string());
        mime_types.insert(
            ".xltm".to_string(),
            "application/vnd.ms-excel.template.macroEnabled.12".to_string(),
        );
        mime_types.insert(
            ".xltx".to_string(),
            "application/vnd.openxmlformats-officedocument.spreadsheetml.template".to_string(),
        );
        mime_types.insert(".xlw".to_string(), "application/vnd.ms-excel".to_string());
        mime_types.insert(".xml".to_string(), "text/xml".to_string());
        mime_types.insert(".xof".to_string(), "x-world/x-vrml".to_string());
        mime_types.insert(".xpm".to_string(), "image/x-xpixmap".to_string());
        mime_types.insert(
            ".xps".to_string(),
            "application/vnd.ms-xpsdocument".to_string(),
        );
        mime_types.insert(".xsd".to_string(), "text/xml".to_string());
        mime_types.insert(".xsf".to_string(), "text/xml".to_string());
        mime_types.insert(".xsl".to_string(), "text/xml".to_string());
        mime_types.insert(".xslt".to_string(), "text/xml".to_string());
        mime_types.insert(".xsn".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".xtp".to_string(), "application/octet-stream".to_string());
        mime_types.insert(".xwd".to_string(), "image/x-xwindowdump".to_string());
        mime_types.insert(".z".to_string(), "application/x-compress".to_string());
        mime_types.insert(
            ".zip".to_string(),
            "application/x-zip-compressed".to_string(),
        );

        MimeDict { mime_types }
    }

    /// Takes a file path as input and returns the corresponding MIME type as an `Option<String>`.
    /// If the file extension is not found in the `mime_types` map, it returns `None`.
    ///
    /// # Arguments
    ///
    /// * `path` - A type that implements `AsRef<str>` representing the file path.
    ///
    /// # Examples
    ///
    /// ```
    /// let mime_dict = mimee::MimeDict::new();
    /// let content_type = mime_dict.get_content_type("example.txt");
    /// assert_eq!(content_type, Some("text/plain".to_string()));
    /// ```
    pub fn get_content_type<T: AsRef<str>>(&self, path: T) -> Option<String> {
        match MimeDict::get_extension(path) {
            Some(ext) => self.mime_types.get(&ext).cloned(),
            None => None,
        }
    }

    /// Extracts the file extension from a given file path and returns it as an `Option<String>`.
    /// The extension is converted to lowercase. If no extension is found, it returns `None`.
    fn get_extension<T: AsRef<str>>(path: T) -> Option<String> {
        match path.as_ref().rfind(".") {
            Some(i) => Some(path.as_ref()[i..].to_string().to_lowercase()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::MimeDict;

    /// Tests that known file extensions return the correct MIME type.
    #[rstest]
    #[case("khown.txt", "text/plain")]
    #[case("khown.html", "text/html")]
    fn known_extensions_return_some(#[case] input: &str, #[case] expected: &str) {
        let mime_dict = MimeDict::new();
        let content_type = &mime_dict.get_content_type(input).unwrap();
        assert_eq!(content_type, expected)
    }

    /// Tests that unknown file extensions return `None`.
    #[rstest]
    #[case("unkhown.xyz")]
    fn unknown_extensions_return_none(#[case] input: &str) {
        let mime_dict = MimeDict::new();
        let result = &mime_dict.get_content_type(input);
        assert_eq!(*result, Option::None);
    }

    /// Tests that file paths with double-dotted extensions are not supported and return `None`.
    #[rstest]
    #[case("known.exe.config")]
    fn double_dotted_extensions_are_not_supported(#[case] input: &str) {
        let mime_dict = MimeDict::new();
        let result = &mime_dict.get_content_type(input);
        assert_eq!(*result, Option::None);
    }

    /// Tests that file paths with double-dotted extensions are not supported and return `None`.
    #[rstest]
    #[case("known.dvr-ms", "video/x-ms-dvr")]
    fn dahsed_extensions_should_be_matched(#[case] input: &str, #[case] expected: &str) {
        let mime_dict = MimeDict::new();
        let content_type = &mime_dict.get_content_type(input).unwrap();
        assert_eq!(content_type, expected);
    }

    /// Tests that dashed extensions are correctly matched to their MIME types.
    #[rstest]
    #[case("/first/example.txt", "text/plain")]
    #[case(r"\second\example.txt", "text/plain")]
    fn both_slash_formats_are_supported(#[case] input: &str, #[case] expected: &str) {
        let mime_dict = MimeDict::new();
        let content_type = &mime_dict.get_content_type(input).unwrap();
        assert_eq!(content_type, expected);
    }

    /// Tests that dots in directory names are ignored when determining the file extension.
    #[rstest]
    #[case(r"first.css/example.txt", "text/plain")]
    #[case(r"\second.css\example.txt", "text/plain")]
    fn dot_in_directories_are_ignored(#[case] input: &str, #[case] expected: &str) {
        let mime_dict = MimeDict::new();
        let content_type = &mime_dict.get_content_type(input).unwrap();
        assert_eq!(content_type, expected);
    }

    /// Tests that invalid characters in file paths are ignored when determining the file extension.
    #[test]
    fn invalid_characters_are_ignored() {
        let mime_dict = MimeDict::new();
        let invalid_chars_str: String = get_invalid_path_chars().iter().collect();
        let path: String = [invalid_chars_str, ".txt".to_string()].concat();
        let content_type = &mime_dict.get_content_type(path).unwrap();
        assert_eq!(content_type, "text/plain");
    }

    fn get_invalid_path_chars() -> Vec<char> {
        vec!['\\', '/', ':', '*', '?', '"', '<', '>', '|']
    }

    /// Tests that non-ASCII characters in file paths are supported and correctly matched to their MIME types.
    #[rstest]
    #[case("Löwe 老虎 Léopard.html", "text/html")]
    fn none_assci_characters_are_supported(#[case] input: &str, #[case] expected: &str) {
        let mime_dict = MimeDict::new();
        let content_type = &mime_dict.get_content_type(input).unwrap();
        assert_eq!(content_type, expected)
    }
}
