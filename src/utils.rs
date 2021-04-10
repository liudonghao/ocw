pub mod lite_json_2_map{

    use lite_json::json::JsonValue;
    use std::collections::HashMap;

    pub struct MVConfig{
        pub parse_string_again:bool,
    }

    
    /// ...
    /// * @author David
    /// * @since 2021/04/05
    /// * @use If there is a conversion path:
    /// * raw JSON -> JsonValue -> X -> HashMap powered by Rust primitives,
    /// * then this is the X, sitting between JsonValue and specificily typed HashMap.
    /// * It's like a dual of JsonValue, but more with Rust primitive and collection types.
    /// ...
    #[derive(Debug)]
    pub enum MapValue{
        I64(i64),
        F64(f64),
        STR(String),
        MAP(HashMap<String, MapValue>),
        BOOL(bool),
        VEC(Vec<MapValue>),
        ERR(&'static str),
        Null
    }

    impl MapValue{
        /*
        This kind of funtion will ease user's pain to utilize MapValue
        */
        pub fn to_string(&self)->String{
            match self{
                MapValue::STR(s)=>s.to_string(),
                _=>"ERROR".to_string()
            }
        }

        pub fn to_vec_string(&self)->Vec<String>{
            let mut vec:Vec<String>=Vec::new();
            match self{
                MapValue::VEC(v)=>{
                    for m in v{
                        let url=m.to_string();
                        vec.push(url);
                    }
                },
                _=>{}
            }
            return vec;
        }

        pub fn get_string_from_map(&self, s:&String)->String{
            match self{
                MapValue::MAP(m)=>{
                    match m.get(s){
                        Some(MapValue::STR(st))=>{
                            return st.to_string();
                        },
                        _=>"ERROR".to_string()
                    }
                },
                _=>"ERROR".to_string()
            }
        }

        pub fn get_u128_from_map(&self, s:&String)->Option<u128>{
            match self{
                MapValue::MAP(m)=>{
                    match m.get(s){
                        Some(MapValue::I64(st))=>{
                            return Some(*st as u128);
                        },
                        _=>None
                    }
                },
                _=>None
            }
        }
    }
     
    /// ...
    /// * @author David
    /// * @since 2021/04/05
    /// * @use Given a string and Config, parse the string using lite_json,
    /// * and return a MapValue which is organized with Vec, HashMap and Rust's primitive types.
    /// * It's more easy to convert MapValue into the Vec or HasMap as expected.
    /// ...
    pub fn json_value_to_map(json_str:&str, config:&MVConfig) ->MapValue{
        let val = lite_json::parse_json(json_str);
        let mut jv;
        match val{
            Ok(o)=>{
                jv=o;
            },
            Err(e)=>{
                return MapValue::ERR("Something wrong in parsing");
            }
        }
        
        return _xx(Some(jv), config);
    }

    /// ...
    /// * @author David
    /// * @since 2021/04/05
    /// * This is the core logics of the intermediate idea,
    /// * all magics rely on this, it's recursive.
    /// ...
    fn _xx(jv:Option<JsonValue>, config:&MVConfig)->MapValue{
        match jv{
            None=>{
                return MapValue::Null;
            },
            Some(JsonValue::Number(n))=>{
                if n.fraction_length==0{
                    return MapValue::I64(n.integer);
                }else{
                    return MapValue::F64(n.to_f64());
                }
            },
            Some(JsonValue::String(s))=>{
                let string:String=s.into_iter().collect();
                if config.parse_string_again {
                    let res2=lite_json::parse_json(string.as_str());
                    match res2{
                        Ok(jv)=>{
                            // This proves it can be parsed deeer.
                            return _xx(Some(jv), config);
                        },
                        Err(e)=>{
                            return MapValue::STR(string);
                        }
                    }
                }else{
                    return MapValue::STR(string);
                }
            },
            Some(JsonValue::Boolean(b))=>{
                return MapValue::BOOL(b);
            },
            Some(JsonValue::Array(a))=>{
                let mut vec:Vec<MapValue>=Vec::new();
                for j in a{
                    vec.push(_xx(Some(j), config));
                }
                return MapValue::VEC(vec);
            },
            Some(JsonValue::Object(o))=>{
                let mut map:HashMap<String,MapValue>=HashMap::new();
                for (v, j) in o{
                    let key=v.into_iter().collect();
                    let value=_xx(Some(j), config);
                    map.insert(key, value);
                }
                return MapValue::MAP(map);
            },
            _=>{
                return MapValue::Null;
            }
        }
    }

    // '''''''''''''''''''All below does unit tests''''''''''''''''''''''
    #[test]
    pub fn test_obj(){
        let mut config:MVConfig=MVConfig{
            parse_string_again:false, // not parse string further
        };
        // test object
        let mut json_str:&str="{\"dec\":88.88, \"name\":{\"first\":\"David\", \"last\":\"Liu\"}}";
        let mut j2m=json_value_to_map(json_str, &config);
        match j2m{
            MapValue::MAP(f)=>{
                println!("MAP {:?}", f);
            },
            _=>panic!("MAP failed."),
        }

        //{"sc-address": "5DeeNqcAcaHDSed2HYnqMDK7JHcvxZ5QUE9EKmjc5snvU6wF", "pair-url": "[\"https://bitkeys.work/file/subocw/datasource1.txt\",\"https://bitkeys.work/file/subocw/datasource2.txt\",\"https://bitkeys.work/file/subocw/datasource3.txt\",\"https://bitkeys.work/file/subocw/datasource4.txt\"]"}
        config.parse_string_again=true;
        json_str=r#"{"sc-address": "5DeeNqcAcaHDSed2HYnqMDK7JHcvxZ5QUE9EKmjc5snvU6wF", "pair-url": "[\"https://bitkeys.work/file/subocw/datasource1.txt\",\"https://bitkeys.work/file/subocw/datasource2.txt\",\"https://bitkeys.work/file/subocw/datasource3.txt\",\"https://bitkeys.work/file/subocw/datasource4.txt\"]"}"#;
        j2m=json_value_to_map(json_str, &config);
        match j2m{
            MapValue::MAP(f)=>{
                println!("MAP {:?}", f);
            },
            _=>panic!("Complicated MAP failed."),
        }
    }

    #[test]
    pub fn single_primitive(){
        let config:MVConfig=MVConfig{
            parse_string_again:false, // not parse string further
        };

        // test float number
        let mut json_str:&str="88.88";
        let j2m=json_value_to_map(json_str, &config);
        match j2m{
            MapValue::F64(f)=>{
                assert_eq!(88.88, f);
            },
            _=>panic!("F64 failed."),
        }

        // test integer number
        json_str="88";
        let j2m=json_value_to_map(json_str, &config);
        match j2m{
            MapValue::I64(f)=>{
                assert_eq!(88, f);
            },
            _=>panic!("I64 failed."),
        }

        // test boolean value
        json_str="true";
        let j2m=json_value_to_map(json_str, &config);
        match j2m{
            MapValue::BOOL(f)=>{
                assert_eq!(true, f);
            },
            _=>panic!("BOOL failed."),
        }

        // test string value
        json_str="\"god\"";
        let j2m=json_value_to_map(json_str, &config);
        match j2m{
            MapValue::STR(f)=>{
                assert_eq!("god", f);
            },
            _=>panic!("STR failed."),
        }

    }

    #[test]
    pub fn primitive_array(){
        let config:MVConfig=MVConfig{
            parse_string_again:false, // not parse string further
        };

        // test array of boolean
        let mut json_str="[true, false]";
        let j2m=json_value_to_map(json_str, &config);
        match j2m{
            MapValue::VEC(f)=>{
                println!("====test array of boolean===");
                for jm in f{
                    println!("{:?}", jm);
                }
            },
            _=>panic!("VEC failed."),
        }

        // test array of string
        json_str="[\"one\", \"two\"]";
        let j2m=json_value_to_map(json_str, &config);
        match j2m{
            MapValue::VEC(f)=>{
                println!("====test array of string===");
                for jm in f{
                    println!("{:?}", jm);
                }
            },
            _=>panic!("VEC failed."),
        }
    }

    #[test]
    pub fn test(){
        let json_str:&str="asdf[1,2]";
        let val = lite_json::parse_json(json_str);
        match val{
            Ok(j)=>{
                println!("JV {:?}",j);
            },
            Err(e)=>{
                println!("Err {:?}",e);

            }
        }
    }
}