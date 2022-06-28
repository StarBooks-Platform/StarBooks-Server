using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Text.Json.Serialization;

namespace StarBooks.Domain.Books;

[Table("identifiers")]
public class IndustryIdentifierModel
{
    [JsonIgnore, Key]
    public int IdentifierId { get; set; }
    
    [JsonPropertyName("type")]
    public string Type { get; set; }

    [JsonPropertyName("identifier")]
    public string Identifier { get; set; }
    
    [JsonIgnore]
    public BookModel Book { get; set; }
}
