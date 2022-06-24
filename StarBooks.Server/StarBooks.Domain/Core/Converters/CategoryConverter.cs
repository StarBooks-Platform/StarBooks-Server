using System.Text.Json;
using System.Text.Json.Serialization;
using StarBooks.Domain.Books;

namespace StarBooks.Domain.Core.Converters;

public class CategoryConverter: JsonConverter<CategoryModel>
{
    public override CategoryModel? Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        return CategoryModel.Parse(reader.GetString()!);
    }

    public override void Write(Utf8JsonWriter writer, CategoryModel value, JsonSerializerOptions options)
    {
        writer.WriteStringValue(value.ToString());
    }
}