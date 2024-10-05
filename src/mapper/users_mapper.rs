use rbatis::impl_select;
use crate::pojo::users::Users;

impl_select!(Users{select_by_id(id:String) -> Option => "`where id = #{id} limit 1`"});
impl_select!(Users{login(username:String,password:String) -> Option => "`where username = #{username} AND password = #{password} limit 1`"});
