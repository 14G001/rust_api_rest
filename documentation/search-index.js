var searchIndex = new Map(JSON.parse('[\
["api",{"doc":"","t":"CCCCCCCCCFOONNONNOONNONNNNNNNCCCCCCFNNNNNNNNNNNFNNOOONHNHNNNNKMMMPPGPPPPPPPPPNNNNNNHNNNNNCCFNNNONONNNNHCCCHHHCCHCCCCCNNNNNFNNNNNNNNNFNNNNNNNNNFNNNNNNNNNFNNNNNNNNNFNNNNCHCCCCHCCHPPGNNNNNNNNNNNNFNNNNNNNNNNCCCCHHHHHHHCFNNNNNNNNNNNNNO","n":["client","db","env_var","errors","services","statistics","transaction","utils","client","Client","balance","birth_date","borrow","borrow_mut","client_name","clone","clone_into","country","document_number","fmt","from","id","into","new","to_owned","try_from","try_into","type_id","vzip","class","db","error","ram_db","ram_db","singleton","RamDbConn","add_client","borrow","borrow_mut","from","get_client","into","try_from","try_into","type_id","update_client_balance","vzip","RamDb","borrow","borrow_mut","client_id_to_assign","clients","file_count","from","get_db","into","is_document_already_used","try_from","try_into","type_id","vzip","Database","add_client","get_client","update_client_balance","CantCreateMoreClients","ClientNotExist","DbError","DocNumAlreadyInUse","FileCreation","FileExists","FileOpening","FileReading","FileWriting","InvalidFields","None","UnknownPayWay","borrow","borrow_mut","clone","clone_into","from","into","ok","to_owned","try_from","try_into","type_id","vzip","env_vars","load","EnvVars","borrow","borrow_mut","from","host","into","port","try_from","try_into","type_id","vzip","load_env_vars","bad_req","cases","parse","bad_request","get_err_msg","parsing_error","init","resource","config","client_balance","new_client","new_credit_transaction","new_debit_transaction","store_balances","borrow","borrow_mut","from","into","register","service","try_from","try_into","type_id","vzip","borrow","borrow_mut","from","into","register","service","try_from","try_into","type_id","vzip","borrow","borrow_mut","from","into","register","service","try_from","try_into","type_id","vzip","borrow","borrow_mut","from","into","register","service","try_from","try_into","type_id","vzip","borrow","borrow_mut","from","into","register","service","try_from","try_into","type_id","vzip","balances","generate_result_balances","init_criteria","pay_ways","transaction","tran_by_pay_way","tran_by_pay_way","amt_updater","pay_way","get_new_client_amt","Credit","Debit","PayWay","borrow","borrow_mut","clone","clone_into","fmt","from","into","to_owned","try_from","try_into","type_id","vzip","Transaction","borrow","borrow_mut","from","into","new","process","try_from","try_into","type_id","vzip","date","file","parse","format","get_curr_date_ddmmyyyy","append_file","create_file","file_exists","open_file","overwrite_file","read_file","naive_date","CustomNaiveDate","borrow","borrow_mut","clone","clone_into","deserialize","fmt","from","into","to_owned","try_from","try_into","type_id","vzip","yof"],"q":[[0,"api"],[8,"api::client"],[9,"api::client::client"],[29,"api::db"],[32,"api::db::class"],[33,"api::db::class::ram_db"],[35,"api::db::class::ram_db::ram_db"],[47,"api::db::class::ram_db::singleton"],[61,"api::db::db"],[65,"api::db::error"],[89,"api::env_var"],[91,"api::env_var::env_vars"],[102,"api::env_var::load"],[103,"api::errors"],[106,"api::errors::bad_req"],[107,"api::errors::cases"],[108,"api::errors::parse"],[109,"api::services"],[111,"api::services::init"],[112,"api::services::resource"],[117,"api::services::resource::client_balance"],[127,"api::services::resource::new_client"],[137,"api::services::resource::new_credit_transaction"],[147,"api::services::resource::new_debit_transaction"],[157,"api::services::resource::store_balances"],[167,"api::statistics"],[168,"api::statistics::balances"],[169,"api::transaction"],[172,"api::transaction::init_criteria"],[173,"api::transaction::init_criteria::tran_by_pay_way"],[174,"api::transaction::pay_ways"],[176,"api::transaction::pay_ways::amt_updater"],[177,"api::transaction::pay_ways::pay_way"],[192,"api::transaction::transaction"],[203,"api::utils"],[206,"api::utils::date"],[207,"api::utils::date::format"],[208,"api::utils::file"],[214,"api::utils::parse"],[215,"api::utils::parse::naive_date"],[230,"core::fmt"],[231,"core::fmt"],[232,"core::any"],[233,"actix_http::body::boxed"],[234,"actix_web::response::response"],[235,"alloc::string"],[236,"actix_web::config"],[237,"actix_web::config"],[238,"serde::de"]],"d":["Contiene una estructura con los datos que pueden tener los …","Contiene estructuras y funciones para la persistencia de …","Contiene la estructura en la que se persistirán las …","Contiene los posibles errores que se podrán manejar por …","Contiene funciones para iniciar todos los servicios del …","Contiene funciones para generar los archivos de balances …","Contiene la estructura con los datos de la transacción y …","Contiene funciones de ayuda para simplificar:","Contiene una estructura con los datos que pueden tener los …","Estructura con los datos de cliente de la aplicación.","","","","","","","","","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","Crea un cliente con todos sus datos sin ninguno de sus …","","","","","","Contiene las funciones para poder operar con los distintos …","Contiene las operaciones que deberán tener los distintos …","Contiene una lista con todos los errores posibles de la …","Contiene las operaciones para poder persistir los datos de …","Contiene las operaciones para poder persistir los datos de …","Contiene los mecanismos y estructuras para poder trabajar …","Contiene las operaciones para poder persistir los datos de …","La documentación de la función se encuentra en la …","","","Returns the argument unchanged.","La documentación de la función se encuentra en la …","Calls <code>U::from(self)</code>.","","","","La documentación de la función se encuentra en la …","","Estructura con los datos globales de la base de datos en …","","","","","","Returns the argument unchanged.","Devuelve la base de datos que se encuentra en la memoria …","Calls <code>U::from(self)</code>.","Verifica si el número de documento ya fué utilizado por …","","","","","Contiene las operaciones que deberán tener los distintos …","Agrega un cliente a la base de datos y devuelve el ID que …","Devuelve los datos de un cliente encontrado en la base de …","Actualiza el saldo de un cliente en la base de datos.","","","Contiene todos los posibles errores de procesamiento de …","","","","","","","","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Recibe el resultado de una operación y indica si salió …","","","","","","Estructura con las variables de entorno de la aplicación …","Función que lee las variables de entorno del archivo “…","Estructura con las variables de entorno de la aplicación …","","","Returns the argument unchanged.","","Calls <code>U::from(self)</code>.","","","","","","Esta función lee las variables de entorno del archivo “…","Contiene una función que genera un mensaje de respuesta …","Contiene una función que devuelve un mensaje de error a …","Contiene una función que genera un mensaje de respuesta …","Función que genera un mensaje de respuesta de error HTTP …","Función que devuelve un mensaje de error a partir de un …","Función que genera un mensaje de respuesta de error HTTP …","Contiene una función que inicia todos los servicios del …","Contiene las funciones de cada servicio del servidor y su …","Esta función inicia todos los servicios del servidor y su …","Servicio de la API que recibe un ID de cliente y devuelve …","Servicio de la API que recibe los datos de un cliente y …","Servicio de la API que procesa una transacción de …","Servicio de la API que procesa una transacción de débito …","Servicio de la API que persiste los datos de todos los …","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Servicio de la API que recibe un ID de cliente y devuelve …","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Servicio de la API que recibe los datos de un cliente y …","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Servicio de la API que procesa una transacción de …","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Servicio de la API que procesa una transacción de débito …","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","Servicio de la API que persiste los datos de todos los …","","","","","Contiene una función que persiste los datos de todos los …","Servicio de la API que persiste los datos de todos los …","Contiene funciones para procesar transacciones de maneras …","Contiene los diferentes métodos de pago de las …","Contiene estructura y funciones para procesar las …","Contiene funciones para procesar transacciones en base al …","Procesa una transacción en base al método pago de la …","Contiene una función que actualiza el monto del cliente …","Contiene los posibles métodos de pago de las …","Devuelve el saldo nuevo del cliente en base al monto y …","","","Contiene los posibles métodos de pago de las …","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","","Estructura con los datos de una tranacción.","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","Crea una transacción con todos sus datos sin ninguno de …","Procesa una transacción en base a los parámetros …","","","","","Funciones para obtener fechas en formatos específicos de …","Funciones para trabajar con archivos.","Funciones para copiar datos de Jsons a estructuras custom …","Funciones para obtener fechas en formatos específicos de …","Devuelve","Escribe datos al final de un archivo.","Crea un archivo en el directorio recibido con el contenido …","Verifica si el archivo recibido existe o no.","Abre un archivo con permisos de lectura o escritura segun …","Reemplaza el contenido de un archivo por el contenido …","Lee el contenido de un archivo.","Estructura que permite copiar datos de Json con formato …","Estructura que permite copiar datos de Json con formato …","","","","","","","Returns the argument unchanged.","Calls <code>U::from(self)</code>.","","","","","",""],"i":[0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,30,30,30,30,30,30,30,30,30,30,30,0,10,10,10,10,10,10,0,10,0,10,10,10,10,0,31,31,31,8,8,0,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,0,8,8,8,8,8,0,0,0,13,13,13,13,13,13,13,13,13,13,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,19,19,19,19,19,0,19,19,19,19,21,21,21,21,21,0,21,21,21,21,22,22,22,22,22,0,22,22,22,22,23,23,23,23,23,0,23,23,23,23,24,24,24,24,24,0,24,24,24,24,0,0,0,0,0,0,0,0,0,0,25,25,0,25,25,25,25,25,25,25,25,25,25,25,25,0,26,26,26,26,26,26,26,26,26,26,0,0,0,0,0,0,0,0,0,0,0,0,0,28,28,28,28,28,28,28,28,28,28,28,28,28,28],"f":[0,0,0,0,0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],0,[1,1],[[-1,-2],2,[],[]],0,0,[[1,3],4],[-1,-1,[]],0,[-1,-2,[],[]],[[],1],[-1,-2,[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[-1,-2,[],[]],0,0,0,0,0,0,0,[[7,1],8],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-1,[]],[[1,7],8],[-1,-2,[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[[7,9],8],[-1,-2,[],[]],0,[-1,-2,[],[]],[-1,-2,[],[]],0,0,0,[-1,-1,[]],[[],10],[-1,-2,[],[]],[11,12],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[-1,-2,[],[]],0,[[7,1],8],[[1,7],8],[[7,9],8],0,0,0,0,0,0,0,0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[8,8],[[-1,-2],2,[],[]],[-1,-1,[]],[-1,-2,[],[]],[8,12],[-1,-2,[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[-1,-2,[],[]],0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-1,[]],0,[-1,-2,[],[]],0,[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[-1,-2,[],[]],[[],13],0,0,0,[[14,8],[[16,[15]]]],[8,17],[[],[[16,[15]]]],0,0,[18,2],0,0,0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-1,[]],[-1,-2,[],[]],[[19,20],2],0,[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-1,[]],[-1,-2,[],[]],[[21,20],2],0,[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-1,[]],[-1,-2,[],[]],[[22,20],2],0,[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-1,[]],[-1,-2,[],[]],[[23,20],2],0,[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,-1,[]],[-1,-2,[],[]],[[24,20],2],0,[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[-1,-2,[],[]],0,[[],8],0,0,0,0,[[7,9,25],16],0,0,[[9,9,25],9],0,0,0,[-1,-2,[],[]],[-1,-2,[],[]],[25,25],[[-1,-2],2,[],[]],[[25,3],4],[-1,-1,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[-1,-2,[],[]],0,[-1,-2,[],[]],[-1,-2,[],[]],[-1,-1,[]],[-1,-2,[],[]],[[],26],[[26,7,9,25],8],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[-1,-2,[],[]],0,0,0,0,[[],17],[[27,17],8],[[17,12,14],8],[14,12],[[14,12,12],27],[[14,17],8],[[17,27],8],0,0,[-1,-2,[],[]],[-1,-2,[],[]],[28,28],[[-1,-2],2,[],[]],[-1,[[5,[28]]],29],[[28,3],4],[-1,-1,[]],[-1,-2,[],[]],[-1,-2,[],[]],[-1,[[5,[-2]]],[],[]],[-1,[[5,[-2]]],[],[]],[-1,6,[]],[-1,-2,[],[]],0],"c":[],"p":[[5,"Client",9],[1,"tuple"],[5,"Formatter",230],[8,"Result",230],[6,"Result",231],[5,"TypeId",232],[1,"u8"],[6,"DbError",65],[1,"f64"],[5,"RamDb",47],[1,"usize"],[1,"bool"],[5,"EnvVars",91],[1,"str"],[5,"BoxBody",233],[5,"HttpResponse",234],[5,"String",235],[5,"ServiceConfig",236],[5,"service",117],[5,"AppService",236],[5,"service",127],[5,"service",137],[5,"service",147],[5,"service",157],[6,"PayWay",177],[5,"Transaction",192],[5,"File",237],[5,"CustomNaiveDate",215],[10,"Deserializer",238],[5,"RamDbConn",35],[10,"Database",61]],"b":[]}]\
]'));
if (typeof exports !== 'undefined') exports.searchIndex = searchIndex;
else if (window.initSearch) window.initSearch(searchIndex);
