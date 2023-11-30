use diesel::prelude::*;
use diesel_async::{AsyncPgConnection,RunQueryDsl};

use crate::schema::*;
use crate::models::*;

pub struct RustaceanRepository;

impl RustaceanRepository {
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustacean> {
        rustaceans::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).load(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table)
            .values(new_rustacean)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, rustacean: Rustacean) -> QueryResult<Rustacean> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::name.eq(rustacean.name),
                rustaceans::email.eq(rustacean.email)
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id)).execute(c).await
    }
}


pub struct UserRepository;

impl UserRepository{

    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(c).await
    }

    pub async fn find_by_username(c: &mut AsyncPgConnection, username: &String) -> QueryResult<User>{
        users::table.filter(users::username.eq(username)).get_result(c).await
    }
    pub async fn find_with_roles(c: &mut AsyncPgConnection) -> QueryResult<Vec<(User, Vec<(UserRole,Role)>)>>{
        let users = users::table.load::<User>(c).await?;
        let result = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole,Role)>(c).await?
            .grouped_by(&users);

        Ok(users.into_iter().zip(result).collect())
    }
    pub async fn create(c: &mut AsyncPgConnection, new_user: NewUser, role_codes: Vec<String>) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(new_user)
            .get_result::<User>(c)
            .await?;
        for role_code in role_codes{
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(c, &role_code).await{
                    NewUserRole { user_id:user.id, role_id:role.id}
                } else {
                    let name = role_code.to_string();
                    let new_role = NewRole { code: role_code, name};
                    let role = RoleRepository::create(c,new_role).await?;
                    NewUserRole { user_id:user.id, role_id:role.id}
                }
            };
            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .get_result::<UserRole>(c)
                .await?;
        }
        Ok(user)
    }
    pub async fn delete(c: &mut AsyncPgConnection, id:i32) -> QueryResult<usize>{
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(id))).execute(c).await?;
        diesel::delete(users::table.find(id)).execute(c).await
    }
}

pub struct RoleRepository;

impl RoleRepository{
    pub async fn find_by_ids(c: &mut AsyncPgConnection, ids:Vec<i32>) -> QueryResult<Vec<Role>>{
        roles::table.filter(roles::id.eq_any(ids)).load(c).await
    }
    pub async fn find_by_code(c: &mut AsyncPgConnection, code: &String) -> QueryResult<Role>{
        roles::table.filter(roles::code.eq(code)).first(c).await
    }

    pub async fn find_by_user(c: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>>{
        let user_roles = UserRole::belonging_to(&user).get_results::<UserRole>(c).await?;
        let role_ids: Vec<i32> = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        
        Self::find_by_ids(c, role_ids).await
    }
    pub async fn create(c: &mut AsyncPgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(new_role)
            .get_result(c)
            .await
    }
}