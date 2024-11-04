using System.Runtime.InteropServices;
using System.Text;

namespace FyroxLite;

internal partial struct NativeString
{

    internal NativeString(byte_slice data)
    {
        this.data = data;
    }

    internal static unsafe string ToFacade(in NativeString self)
    {
        return Encoding.UTF8.GetString(self.data.begin, self.data.length);
    }
    
    [ThreadStatic]
    private static byte[]? _buffer;

    internal static NativeString FromFacade(in string self)
    {
        _buffer ??= new byte[1024 * 1024];
        
        var bytes = Encoding.UTF8.GetBytes(self, 0, self.Length, _buffer, 0);
        
        unsafe
        {
            fixed (byte* buffer_ptr = _buffer)
            {
                var native_slice = byte_slice.fyrox_lite_upload_u8_slice(new byte_slice(buffer_ptr, bytes));
                return new NativeString(native_slice);
            }
        }
    }
}