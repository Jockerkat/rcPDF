# PDF Basic Objects UML Class Diagram

The dashed lines in the UML diagram below represent implementations (i.e., `impl SourceTrait for TargetStruct`) while the solid lines represent inheritance (i.e., `impl TargetTrait for SourceStruct` or `trait SourceTrait : TargetTrait`).

```mermaid
classDiagram
    direction RL

    Display<|--Object
    Object<|--BooleanObject
    IndirectObject<|--StringObject
    LiteralString<|..IndirectObject
    HexadecimalString<|..IndirectObject
    Object<|--NumericObject
    NumericObject<|--IntegerObject
    NumericObject<|--RealObject
    Object<|--StringObject
    StringObject<|--LiteralString
    StringObject<|--HexadecimalString
    Object<|--NameObject
    Object<|--ArrayObject
    Object<|--DictionaryObject
    Object<|--StreamObject
    Object<|--NullObject
    Object<|--IndirectObject
    BooleanObject<|..IndirectObject
    IndirectObject<|--NumericObject
    IntegerObject<|..IndirectObject
    RealObject<|..IndirectObject
    NameObject<|..IndirectObject
    ArrayObject<|..IndirectObject
    DictionaryObject<|..IndirectObject
    StreamObject<|..IndirectObject
    NullObject<|..IndirectObject
    
    class Display {
        <<trait>>
    }

    class Object {
        <<trait>>
    }
    
    class IndirectObject {
        <<trait>>
        +object_number(&self) u32
        +generation_number(&self) u32
        +indirect_reference(&self) String
        +indirect_definition(&self) String
    }
    
    class BooleanObject {
        +new(bool)
    }
    
    class NumericObject {
        <<trait>>
    }
    class IntegerObject {
        +new(i32)
    }
    class RealObject {
        +new(f32)
        -sanitise_input(f32) f32
    }
    
    class StringObject {
        <<trait>>
    }
    class LiteralString {
        +new(String)
        -sanitise_input(AsRef~str~) String
        -string_to_octal(AsRef~str~) String
    }
    class HexadecimalString {
        +new(Into~String~)
        +new_from_hexadecimal(Into~String~)
        -string_to_hexadecimal(AsRef~str~) String
        -is_valid_hexadecimal(AsRef~str~) bool
    }
    
    class NameObject {
        +new(Into~String~)
        -sanitiseInput(AsRef~str~) String
    }
    class ArrayObject {
        +new()
        +new_from_into_iterator(IntoIterator~Item = impl Object~)
        +push(&mut self, impl Object)
    }
    class DictionaryObject {
        +new()
        +new_from_hashmap(HashMap~NameObject, Box~dyn Object~~)
        +insert(&mut self, NameObject, impl Object) Option~Box~dyn Object~~
        +get(&self, &NameObject) Option~&Box~dyn Object~~
    }
    class StreamObject {
        +new()
        +new_from_stream(Into~String~, i32)
        +new_from_dictionary_stream(DictionaryObject, Into~String~)
    }
    
    class NullObject {
        +new()
    }
```

The `Object` trait is an umbrella trait for all basic PDF objects and represents a **direct object**.
Every basic object implements this trait.
Every basic object can be indirectly referenced as they implement the `IndirectObject` trait.
`IndirectObject` implementors contain `object_number` and `generation_number` struct fields; the usage of the indirect reference is not guaranteed.

All `new()` constructors should have an `global_object_number_counter` parameter so that testing becomes possible (directly using the atomic counter defined in `lib.rs` in the struct instantiation, as done before, required all object tests to run in a specific order; passing the counter as a parameter allows for an atomic counter to be defined per test module, circumventing the need of a specific test order).

## Display Trait Implementation

The implementation of the `Display` trait is defined in the respective subsections of the objects.
