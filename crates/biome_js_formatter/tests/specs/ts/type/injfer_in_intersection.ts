type Type<T> = [T] extends [(infer S extends string) & {}] ? S : T;