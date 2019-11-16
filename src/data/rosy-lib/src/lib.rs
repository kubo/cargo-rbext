use rosy::prelude::*;

// equivalent ruby code:
//
//   module @EXT_CLASSNAME@
//     class MyClass
//       def hello(name)
//         puts "Hello, #{name}!"
//       end
//
//       def self.meta
//         hash = Hash.new()
//         hash["api_version"] = ...;
//         hash["copyright"] = RUBY_COPYRIGHT;
//         hash["description"] = RUBY_DESCRIPTION;
//         hash["engine"] = RUBY_ENGINE;
//         hash["platform"] = RUBY_PLATFORM;
//         hash["release_date"] = RUBY_RELEASE_DATE;
//         hash["version"] = RUBY_VERSION;
//         hash
//       end
//     end
//   end

extern "C" fn foo1_s_meta(_class: Class) -> AnyObject {
    let hash = Hash::<String, AnyObject>::new();
    unsafe {
        let ver = Array::<Integer>::new();
        let (major, minor, teeny) = rosy::meta::api_version();
        ver.push(major.into());
        ver.push(minor.into());
        ver.push(teeny.into());
        hash.insert("api_version", ver);
        hash.insert("copyright", rosy::meta::copyright_str());
        hash.insert("description", rosy::meta::description_str());
        hash.insert("engine", rosy::meta::engine_str());
        hash.insert("platform", rosy::meta::platform_str());
        hash.insert("release_date", rosy::meta::release_date_str());
        hash.insert("version", rosy::meta::version_str());
    }
    hash.into()
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_@EXT_BASENAME@() {
    let module = Module::get_or_def("@EXT_CLASSNAME@").unwrap();
    let class = module.def_class("MyClass").unwrap();
    rosy::def_method!(class, "hello", |_this, name| {
        println!("Hello, {}!", name);
    })
    .unwrap();
    let foo1_s_meta: extern "C" fn(_) -> _ = foo1_s_meta;
    class.def_singleton_method("meta", foo1_s_meta).unwrap();
}
