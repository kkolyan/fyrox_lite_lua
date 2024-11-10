namespace FyroxLite;

internal static class ThreadUtils
{
    public static T GetInRightThread<T>(this T? value)
    {
        return value ?? throw new Exception("Fyrox Lite API could be used only in the main thread.");
    }
}