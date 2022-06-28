using System.Text.Json;
using System.Text.Json.Serialization;

namespace StarBooks.Domain.Core.Converters;

public class DateConverter : JsonConverter<DateTime>
{
    public override DateTime Read(
        ref Utf8JsonReader reader,
        Type typeToConvert,
        JsonSerializerOptions options
    )
    {
        var dateString = reader.GetString();
        var dateArray = dateString!.Split('-').Select(int.Parse).ToArray();
        return dateArray.Length != 3
          ? new DateTime(dateArray[0], 1, 1)
          : new DateTime(dateArray[0], dateArray[1], dateArray[2]);
    }

    public override void Write(Utf8JsonWriter writer, DateTime value, JsonSerializerOptions options)
    {
        writer.WriteStringValue(value.ToString("yyyy-MM-dd"));
    }
}
