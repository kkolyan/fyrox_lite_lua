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
        var __item = value;
        var __item_from_facade = __item;
        return new bool_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct bool_slice
{
    private unsafe bool* begin;
    private int length;
    internal List<bool>? Fetched;

    internal unsafe bool_slice(bool* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref bool_slice self)
    {
        var fetched = new List<bool>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<bool> ToFacade(in bool_slice self)
    {
        var fetched = new List<bool>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static bool_slice FromFacade(in List<bool> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

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
        var __item = value;
        var __item_from_facade = __item;
        return new byte_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct byte_slice
{
    private unsafe byte* begin;
    private int length;
    internal List<byte>? Fetched;

    internal unsafe byte_slice(byte* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref byte_slice self)
    {
        var fetched = new List<byte>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<byte> ToFacade(in byte_slice self)
    {
        var fetched = new List<byte>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static byte_slice FromFacade(in List<byte> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

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
        var __item = value;
        var __item_from_facade = __item;
        return new int_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct int_slice
{
    private unsafe int* begin;
    private int length;
    internal List<int>? Fetched;

    internal unsafe int_slice(int* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref int_slice self)
    {
        var fetched = new List<int>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<int> ToFacade(in int_slice self)
    {
        var fetched = new List<int>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static int_slice FromFacade(in List<int> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

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
        var __item = value;
        var __item_from_facade = __item;
        return new long_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct long_slice
{
    private unsafe long* begin;
    private int length;
    internal List<long>? Fetched;

    internal unsafe long_slice(long* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref long_slice self)
    {
        var fetched = new List<long>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<long> ToFacade(in long_slice self)
    {
        var fetched = new List<long>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static long_slice FromFacade(in List<long> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

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
        var __item = value;
        var __item_from_facade = __item;
        return new float_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct float_slice
{
    private unsafe float* begin;
    private int length;
    internal List<float>? Fetched;

    internal unsafe float_slice(float* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref float_slice self)
    {
        var fetched = new List<float>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<float> ToFacade(in float_slice self)
    {
        var fetched = new List<float>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static float_slice FromFacade(in List<float> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

}

[StructLayout(LayoutKind.Explicit)]
internal struct dobule_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private dobule value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe dobule ToFacade(in dobule_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static dobule_result FromFacade(in dobule self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new dobule_result {ok = 1, value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct dobule_optional
{
    private dobule value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static dobule? ToFacade(in dobule_optional value)
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
    public static dobule_optional FromFacade(in dobule? value)
    {
        if (value == null)
        {
            return new dobule_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new dobule_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct dobule_slice
{
    private unsafe dobule* begin;
    private int length;
    internal List<dobule>? Fetched;

    internal unsafe dobule_slice(dobule* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref dobule_slice self)
    {
        var fetched = new List<dobule>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<dobule> ToFacade(in dobule_slice self)
    {
        var fetched = new List<dobule>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static dobule_slice FromFacade(in List<dobule> self)
    {
        // __item
        throw new Exception("slice serialization not implemented yet");
    }

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
        return new NativeString_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeString_slice
{
    private unsafe NativeString* begin;
    private int length;
    internal List<string>? Fetched;

    internal unsafe NativeString_slice(NativeString* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref NativeString_slice self)
    {
        var fetched = new List<string>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = NativeString.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<string> ToFacade(in NativeString_slice self)
    {
        var fetched = new List<string>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = NativeString.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static NativeString_slice FromFacade(in List<string> self)
    {
        // NativeString.FromFacade(__item)
        throw new Exception("slice serialization not implemented yet");
    }

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
internal struct UserScript_slice
{
    private unsafe UserScript* begin;
    private int length;
    internal List<object>? Fetched;

    internal unsafe UserScript_slice(UserScript* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe void Fetch(ref UserScript_slice self)
    {
        var fetched = new List<object>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = UserScript.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        self.Fetched = fetched;
    }

    internal static unsafe List<object> ToFacade(in UserScript_slice self)
    {
        var fetched = new List<object>();
        for (int i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = UserScript.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    internal static UserScript_slice FromFacade(in List<object> self)
    {
        // UserScript.FromFacade(__item)
        throw new Exception("slice serialization not implemented yet");
    }

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