pub trait GraphqlMapper<Entity, GraphObject, Input> {
    // Map an Entity to a Presenter
    fn to_object(entity: Entity) -> GraphObject;

    // Map a Payload to an Entity
    fn to_entity(input: Input) -> Entity;
}