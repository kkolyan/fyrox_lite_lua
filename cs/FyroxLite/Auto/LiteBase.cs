// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite;
using System.Numerics;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;


[StructLayout(LayoutKind.Explicit)]
internal struct NativeBool_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private NativeBool value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe bool ToFacade(in NativeBool_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = NativeBool.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static NativeBool_result FromFacade(in bool self)
    {
        var __item = self;
        var __item_from_facade = NativeBool.FromFacade(__item);
        return new NativeBool_result {ok = 1, value = __item_from_facade};
    }

    internal static NativeBool_result FromFacadeError(in string err)
    {
        return new NativeBool_result {ok = 0, err = NativeString.FromFacade(err)};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeBool_optional
{
    private NativeBool value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static bool? ToFacade(in NativeBool_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = NativeBool.ToFacade(__item);
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static NativeBool_optional FromFacade(in bool? value)
    {
        if (value == null)
        {
            return new NativeBool_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = NativeBool.FromFacade(__item);
        return new NativeBool_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeBool_slice
{
    internal unsafe NativeBool* begin;
    internal int length;

    internal unsafe NativeBool_slice(NativeBool* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<bool> ToFacade(in NativeBool_slice self)
    {
        var fetched = new List<bool>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = NativeBool.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static NativeBool[]? _uploadBuffer;

    internal static NativeBool_slice FromFacade(in List<bool> self)
    {
        _uploadBuffer ??= new NativeBool[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new NativeBool[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = NativeBool.FromFacade(__item);
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (NativeBool* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_bool_slice(new NativeBool_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial NativeBool_slice fyrox_lite_upload_bool_slice(NativeBool_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct byte_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private byte value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe byte ToFacade(in byte_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static byte_result FromFacade(in byte self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new byte_result {ok = 1, value = __item_from_facade};
    }

    internal static byte_result FromFacadeError(in string err)
    {
        return new byte_result {ok = 0, err = NativeString.FromFacade(err)};
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

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial byte_slice fyrox_lite_upload_u8_slice(byte_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct int_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private int value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe int ToFacade(in int_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static int_result FromFacade(in int self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new int_result {ok = 1, value = __item_from_facade};
    }

    internal static int_result FromFacadeError(in string err)
    {
        return new int_result {ok = 0, err = NativeString.FromFacade(err)};
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

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial int_slice fyrox_lite_upload_i32_slice(int_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct long_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private long value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe long ToFacade(in long_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static long_result FromFacade(in long self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new long_result {ok = 1, value = __item_from_facade};
    }

    internal static long_result FromFacadeError(in string err)
    {
        return new long_result {ok = 0, err = NativeString.FromFacade(err)};
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

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial long_slice fyrox_lite_upload_i64_slice(long_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct float_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private float value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe float ToFacade(in float_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static float_result FromFacade(in float self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new float_result {ok = 1, value = __item_from_facade};
    }

    internal static float_result FromFacadeError(in string err)
    {
        return new float_result {ok = 0, err = NativeString.FromFacade(err)};
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

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial float_slice fyrox_lite_upload_f32_slice(float_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct double_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private double value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe double ToFacade(in double_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static double_result FromFacade(in double self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new double_result {ok = 1, value = __item_from_facade};
    }

    internal static double_result FromFacadeError(in string err)
    {
        return new double_result {ok = 0, err = NativeString.FromFacade(err)};
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

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial double_slice fyrox_lite_upload_f64_slice(double_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct NativeString_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private NativeString value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe string ToFacade(in NativeString_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = NativeString.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static NativeString_result FromFacade(in string self)
    {
        var __item = self;
        var __item_from_facade = NativeString.FromFacade(__item);
        return new NativeString_result {ok = 1, value = __item_from_facade};
    }

    internal static NativeString_result FromFacadeError(in string err)
    {
        return new NativeString_result {ok = 0, err = NativeString.FromFacade(err)};
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

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial NativeString_slice fyrox_lite_upload_String_slice(NativeString_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct NativeInstanceId_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private NativeInstanceId value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe object ToFacade(in NativeInstanceId_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = NativeInstanceId.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static NativeInstanceId_result FromFacade(in object self)
    {
        var __item = self;
        var __item_from_facade = NativeInstanceId.FromFacade(__item);
        return new NativeInstanceId_result {ok = 1, value = __item_from_facade};
    }

    internal static NativeInstanceId_result FromFacadeError(in string err)
    {
        return new NativeInstanceId_result {ok = 0, err = NativeString.FromFacade(err)};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeInstanceId_optional
{
    private NativeInstanceId value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static object? ToFacade(in NativeInstanceId_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = NativeInstanceId.ToFacade(__item);
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static NativeInstanceId_optional FromFacade(in object? value)
    {
        if (value == null)
        {
            return new NativeInstanceId_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = NativeInstanceId.FromFacade(__item);
        return new NativeInstanceId_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeInstanceId_slice
{
    internal unsafe NativeInstanceId* begin;
    internal int length;

    internal unsafe NativeInstanceId_slice(NativeInstanceId* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<object> ToFacade(in NativeInstanceId_slice self)
    {
        var fetched = new List<object>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = NativeInstanceId.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static NativeInstanceId[]? _uploadBuffer;

    internal static NativeInstanceId_slice FromFacade(in List<object> self)
    {
        _uploadBuffer ??= new NativeInstanceId[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new NativeInstanceId[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = NativeInstanceId.FromFacade(__item);
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (NativeInstanceId* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_crate_UserScriptImpl_slice(new NativeInstanceId_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial NativeInstanceId_slice fyrox_lite_upload_crate_UserScriptImpl_slice(NativeInstanceId_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeScriptMetadata_slice
{
    internal unsafe NativeScriptMetadata* begin;
    internal int length;

    internal unsafe NativeScriptMetadata_slice(NativeScriptMetadata* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<NativeScriptMetadata> ToFacade(in NativeScriptMetadata_slice self)
    {
        var fetched = new List<NativeScriptMetadata>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static NativeScriptMetadata[]? _uploadBuffer;

    internal static NativeScriptMetadata_slice FromFacade(in List<NativeScriptMetadata> self)
    {
        _uploadBuffer ??= new NativeScriptMetadata[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new NativeScriptMetadata[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (NativeScriptMetadata* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_NativeScriptMetadata_slice(new NativeScriptMetadata_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial NativeScriptMetadata_slice fyrox_lite_upload_NativeScriptMetadata_slice(NativeScriptMetadata_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeScriptProperty_slice
{
    internal unsafe NativeScriptProperty* begin;
    internal int length;

    internal unsafe NativeScriptProperty_slice(NativeScriptProperty* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<NativeScriptProperty> ToFacade(in NativeScriptProperty_slice self)
    {
        var fetched = new List<NativeScriptProperty>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static NativeScriptProperty[]? _uploadBuffer;

    internal static NativeScriptProperty_slice FromFacade(in List<NativeScriptProperty> self)
    {
        _uploadBuffer ??= new NativeScriptProperty[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new NativeScriptProperty[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (NativeScriptProperty* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_NativeScriptProperty_slice(new NativeScriptProperty_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial NativeScriptProperty_slice fyrox_lite_upload_NativeScriptProperty_slice(NativeScriptProperty_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeValue_slice
{
    internal unsafe NativeValue* begin;
    internal int length;

    internal unsafe NativeValue_slice(NativeValue* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<NativeValue> ToFacade(in NativeValue_slice self)
    {
        var fetched = new List<NativeValue>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static NativeValue[]? _uploadBuffer;

    internal static NativeValue_slice FromFacade(in List<NativeValue> self)
    {
        _uploadBuffer ??= new NativeValue[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new NativeValue[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (NativeValue* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_NativeValue_slice(new NativeValue_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial NativeValue_slice fyrox_lite_upload_NativeValue_slice(NativeValue_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct NativePropertyValue_slice
{
    internal unsafe NativePropertyValue* begin;
    internal int length;

    internal unsafe NativePropertyValue_slice(NativePropertyValue* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<NativePropertyValue> ToFacade(in NativePropertyValue_slice self)
    {
        var fetched = new List<NativePropertyValue>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static NativePropertyValue[]? _uploadBuffer;

    internal static NativePropertyValue_slice FromFacade(in List<NativePropertyValue> self)
    {
        _uploadBuffer ??= new NativePropertyValue[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new NativePropertyValue[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (NativePropertyValue* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_NativePropertyValue_slice(new NativePropertyValue_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial NativePropertyValue_slice fyrox_lite_upload_NativePropertyValue_slice(NativePropertyValue_slice managed);
}

[StructLayout(LayoutKind.Explicit)]
internal struct NativeInstanceId_optional_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private NativeInstanceId_optional value;

    [FieldOffset(sizeof(int))]
    private NativeString err;

    internal static unsafe object? ToFacade(in NativeInstanceId_optional_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = NativeInstanceId_optional.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.err));
    }

    internal static NativeInstanceId_optional_result FromFacade(in object? self)
    {
        var __item = self;
        var __item_from_facade = NativeInstanceId_optional.FromFacade(__item);
        return new NativeInstanceId_optional_result {ok = 1, value = __item_from_facade};
    }

    internal static NativeInstanceId_optional_result FromFacadeError(in string err)
    {
        return new NativeInstanceId_optional_result {ok = 0, err = NativeString.FromFacade(err)};
    }
}