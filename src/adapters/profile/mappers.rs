
use crate::adapters::profile::objects::{Profile, ProfileDetail, SimpleProfile};
use crate::domain::entities::{ProfileDetailsEntity, ProfileEntity};
use crate::domain::mappers::graphql_mapper::GraphqlMapper;
use crate::infrastructure::aws::CloudFrontSigner;

pub struct ProfileMapper;


impl GraphqlMapper<ProfileEntity, Profile, String> for ProfileMapper {
    fn to_object(entity: ProfileEntity) -> Profile {
        Profile {
            id: entity.user_id.into(),
            username: entity.username,
            names: entity.names,
            photo: CloudFrontSigner::sing(entity.photo),
            photo_hash: entity.photo_hash,
            portrait: CloudFrontSigner::sing(entity.portrait),
            portrait_hash: entity.portrait_hash,
            description: entity.description,
            verified: entity.verified,
            privacy: entity.privacy,
        }
    }

    fn to_entity(_input: String) -> ProfileEntity {
        todo!()
    }
}


pub struct SimpleProfileMapper;


impl GraphqlMapper<Profile, SimpleProfile, String> for SimpleProfileMapper {
    fn to_object(entity: Profile) -> SimpleProfile {
        SimpleProfile {
            id: entity.id,
            username: entity.username,
            names: entity.names,
            photo: entity.photo,
            photo_hash: entity.photo_hash,
            verified: entity.verified,
            privacy: entity.privacy,
        }
    }

    fn to_entity(_input: String) -> Profile {
        todo!()
    }
}


pub struct ProfileDetailMapper;


impl GraphqlMapper<ProfileDetailsEntity, ProfileDetail, String> for ProfileDetailMapper {
    fn to_object(entity: ProfileDetailsEntity) -> ProfileDetail {
        ProfileDetail {
            id: entity.user_id.into(),
            lat: entity.lat,
            lng: entity.lng,
        }
    }

    fn to_entity(_input: String) -> ProfileDetailsEntity {
        todo!()
    }
}

