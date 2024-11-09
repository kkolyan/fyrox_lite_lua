namespace FyroxLite;

internal partial struct NativeHandle
{
    public override string ToString()
    {
        // equivalent of Handle::decode_from_u128
        
        var index = (uint) low;
        var gen = (uint)(low >> 32);
        return $"{index}:{gen}";
    }
}