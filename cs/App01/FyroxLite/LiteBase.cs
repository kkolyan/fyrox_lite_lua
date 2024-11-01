using FyroxLite.LiteInput;
using FyroxLite.LiteLog;
using FyroxLite.LiteMath;
using FyroxLite.LiteNode;
using FyroxLite.LitePhysics;
using FyroxLite.LitePlugin;
using FyroxLite.LitePrefab;
using FyroxLite.LiteScene;
using FyroxLite.LiteUi;
using FyroxLite.LiteWindow;
using System.Numerics;
using FyroxLite.LiteBase;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;


[StructLayout(LayoutKind.Explicit)]
internal struct bool_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private bool value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe bool ToFacade(in bool_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static bool_result FromFacade(in bool self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new bool_result {ok = 1, value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct bool_optional
{
    private bool value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static bool? ToFacade(in bool_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static bool_optional FromFacade(in bool? value)
    {
        if (value == null)
        {
            return new bool_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new bool_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct bool_slice
{
    internal unsafe bool* begin;
    internal int length;

    internal unsafe bool_slice(bool* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<bool> ToFacade(in bool_slice self)
    {
        var fetched = new List<bool>();
        
        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static bool[]? _uploadBuffer;

    internal static bool_slice FromFacade(in List<bool> self)
    {
        _uploadBuffer ??= new bool[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new bool[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (bool* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_bool_slice(new bool_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial bool_slice fyrox_lite_upload_bool_slice(bool_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct byte_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private byte value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe byte ToFacade(in byte_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static byte_result FromFacade(in byte self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new byte_result {ok = 1, value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct byte_optional
{
    private byte value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static byte? ToFacade(in byte_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static byte_optional FromFacade(in byte? value)
    {
        if (value == null)
        {
            return new byte_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new byte_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct byte_slice
{
    internal unsafe byte* begin;
    internal int length;

    internal unsafe byte_slice(byte* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<byte> ToFacade(in byte_slice self)
    {
        var fetched = new List<byte>();
        
        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static byte[]? _uploadBuffer;

    internal static byte_slice FromFacade(in List<byte> self)
    {
        _uploadBuffer ??= new byte[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new byte[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (byte* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_u8_slice(new byte_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial byte_slice fyrox_lite_upload_u8_slice(byte_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct int_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private int value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe int ToFacade(in int_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static int_result FromFacade(in int self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new int_result {ok = 1, value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct int_optional
{
    private int value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static int? ToFacade(in int_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static int_optional FromFacade(in int? value)
    {
        if (value == null)
        {
            return new int_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new int_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct int_slice
{
    internal unsafe int* begin;
    internal int length;

    internal unsafe int_slice(int* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<int> ToFacade(in int_slice self)
    {
        var fetched = new List<int>();
        
        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static int[]? _uploadBuffer;

    internal static int_slice FromFacade(in List<int> self)
    {
        _uploadBuffer ??= new int[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new int[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (int* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_i32_slice(new int_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial int_slice fyrox_lite_upload_i32_slice(int_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct long_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private long value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe long ToFacade(in long_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static long_result FromFacade(in long self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new long_result {ok = 1, value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct long_optional
{
    private long value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static long? ToFacade(in long_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static long_optional FromFacade(in long? value)
    {
        if (value == null)
        {
            return new long_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new long_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct long_slice
{
    internal unsafe long* begin;
    internal int length;

    internal unsafe long_slice(long* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<long> ToFacade(in long_slice self)
    {
        var fetched = new List<long>();
        
        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static long[]? _uploadBuffer;

    internal static long_slice FromFacade(in List<long> self)
    {
        _uploadBuffer ??= new long[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new long[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (long* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_i64_slice(new long_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial long_slice fyrox_lite_upload_i64_slice(long_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct float_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private float value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe float ToFacade(in float_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static float_result FromFacade(in float self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new float_result {ok = 1, value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct float_optional
{
    private float value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static float? ToFacade(in float_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static float_optional FromFacade(in float? value)
    {
        if (value == null)
        {
            return new float_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new float_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct float_slice
{
    internal unsafe float* begin;
    internal int length;

    internal unsafe float_slice(float* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<float> ToFacade(in float_slice self)
    {
        var fetched = new List<float>();
        
        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static float[]? _uploadBuffer;

    internal static float_slice FromFacade(in List<float> self)
    {
        _uploadBuffer ??= new float[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new float[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (float* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_f32_slice(new float_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial float_slice fyrox_lite_upload_f32_slice(float_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct double_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private double value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe double ToFacade(in double_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static double_result FromFacade(in double self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new double_result {ok = 1, value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct double_optional
{
    private double value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static double? ToFacade(in double_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static double_optional FromFacade(in double? value)
    {
        if (value == null)
        {
            return new double_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new double_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct double_slice
{
    internal unsafe double* begin;
    internal int length;

    internal unsafe double_slice(double* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<double> ToFacade(in double_slice self)
    {
        var fetched = new List<double>();
        
        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static double[]? _uploadBuffer;

    internal static double_slice FromFacade(in List<double> self)
    {
        _uploadBuffer ??= new double[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new double[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (double* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_f64_slice(new double_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial double_slice fyrox_lite_upload_f64_slice(double_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct NativeString_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private NativeString value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe string ToFacade(in NativeString_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = NativeString.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static NativeString_result FromFacade(in string self)
    {
        var __item = self;
        var __item_from_facade = NativeString.FromFacade(__item);
        return new NativeString_result {ok = 1, value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeString_optional
{
    private NativeString value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static string? ToFacade(in NativeString_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = NativeString.ToFacade(__item);
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static NativeString_optional FromFacade(in string? value)
    {
        if (value == null)
        {
            return new NativeString_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = NativeString.FromFacade(__item);
        return new NativeString_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeString_slice
{
    internal unsafe NativeString* begin;
    internal int length;

    internal unsafe NativeString_slice(NativeString* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<string> ToFacade(in NativeString_slice self)
    {
        var fetched = new List<string>();
        
        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = NativeString.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static NativeString[]? _uploadBuffer;

    internal static NativeString_slice FromFacade(in List<string> self)
    {
        _uploadBuffer ??= new NativeString[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new NativeString[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = NativeString.FromFacade(__item);
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (NativeString* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_String_slice(new NativeString_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeString_slice fyrox_lite_upload_String_slice(NativeString_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct UserScript_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private UserScript value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe object ToFacade(in UserScript_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = UserScript.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static UserScript_result FromFacade(in object self)
    {
        var __item = self;
        var __item_from_facade = UserScript.FromFacade(__item);
        return new UserScript_result {ok = 1, value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct UserScript_optional
{
    private UserScript value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static object? ToFacade(in UserScript_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = UserScript.ToFacade(__item);
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static UserScript_optional FromFacade(in object? value)
    {
        if (value == null)
        {
            return new UserScript_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = UserScript.FromFacade(__item);
        return new UserScript_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct UserScript_slice
{
    internal unsafe UserScript* begin;
    internal int length;

    internal unsafe UserScript_slice(UserScript* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<object> ToFacade(in UserScript_slice self)
    {
        var fetched = new List<object>();
        
        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = UserScript.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static UserScript[]? _uploadBuffer;

    internal static UserScript_slice FromFacade(in List<object> self)
    {
        _uploadBuffer ??= new UserScript[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new UserScript[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = UserScript.FromFacade(__item);
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (UserScript* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_crate_UserScriptImpl_slice(new UserScript_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial UserScript_slice fyrox_lite_upload_crate_UserScriptImpl_slice(UserScript_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct UserScript_optional_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private UserScript_optional value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe object? ToFacade(in UserScript_optional_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = UserScript_optional.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static UserScript_optional_result FromFacade(in object? self)
    {
        var __item = self;
        var __item_from_facade = UserScript_optional.FromFacade(__item);
        return new UserScript_optional_result {ok = 1, value = __item_from_facade};
    }
}