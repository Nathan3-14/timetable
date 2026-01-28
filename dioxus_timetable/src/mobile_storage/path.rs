#[cfg(target_os = "android")]
pub fn files_dir() -> std::path::PathBuf {
    use jni::objects::{JObject, JString};
    use jni::JNIEnv;
    use std::sync::mpsc::channel;

    let (tx, rx) = channel();

    dioxus::mobile::wry::prelude::dispatch(
        move |env: &mut JNIEnv, activity: &JObject, _webview| {
            let files_dir = env
                .call_method(activity, "getFilesDir", "()Ljava/io/File;", &[])
                .unwrap()
                .l()
                .unwrap();

            let abs_path = env
                .call_method(files_dir, "getAbsolutePath", "()Ljava/lang/String;", &[])
                .unwrap()
                .l()
                .unwrap();

            let abs_path: JString = abs_path.into();
            let abs_path: String = env.get_string(&abs_path).unwrap().into();

            tx.send(std::path::PathBuf::from(abs_path)).unwrap();
        },
    );

    // tracing::info!("{}", rx.recv().unwrap().display());
    rx.recv().unwrap()
}

#[cfg(target_os = "ios")]
pub fn files_dir() -> std::path::PathBuf {
    use std::path::PathBuf;

    let home = std::env::var("HOME").expect("HOME not set on iOS");
    std::path::PathBuf::from(home).join("Documents")
}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub fn files_dir() -> std::path::PathBuf {
    std::env::current_dir().expect("Failed to get current directory")
}
